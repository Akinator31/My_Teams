use libs::ClientLog;
use crate::client::io_manager::{Context, IoManager};
use crate::transport::{print_colored_reply, reply_code};
use crate::utils::parsing::parse_quoted_segments;

fn parse_created_payload(reply: &str) -> Option<(String, Option<i64>)> {
    let payload = reply.strip_prefix("201 Created.")?.trim();
    let mut parts = payload.split_whitespace();
    let created_uuid = parts.next()?.trim_matches('"').to_string();
    let created_timestamp = parts.next().map(|raw| raw.trim_matches('"').parse::<i64>().unwrap_or(0));
    Some((created_uuid, created_timestamp))
}

pub fn create(io_manager: &mut IoManager, args: &str) {
    let line: String = format!("CREATE {}", args);
    if let Err(e) = io_manager.write_line(&*line) {
        println!("Error occurred while writing to stream: {}", e);
        return;
    }

    let reply = loop {
        let line = match io_manager.read_line() {
            Ok(line) => line,
            Err(e) => {
                println!("Error while reading from stream: {}", e);
                return;
            }
        };
        if line.trim().starts_with("EVENT") {
            crate::events::events::events(&line);
            continue;
        }
        break line;
    };
    match (reply_code(&reply), &io_manager.context) {
        (Some(201), Context::None) => {
            print_colored_reply(&reply);
            let (team_uuid, _) = match parse_created_payload(&reply) {
                Some(payload) => payload,
                None => return,
            };
              let args_parts = parse_quoted_segments(args);
            if args_parts.len() < 2 {
                return;
            }
            ClientLog::client_print_team_created(team_uuid, args_parts[0].clone(), args_parts[1].clone());
        }
        (Some(201), Context::Team(_team_uuid)) => {
            print_colored_reply(&reply);
            let (channel_uuid, _) = match parse_created_payload(&reply) {
                Some(payload) => payload,
                None => return,
            };
            let args_parts = parse_quoted_segments(args);
            if args_parts.len() < 2 { return };
            ClientLog::client_print_channel_created(channel_uuid, args_parts[0].clone(), args_parts[1].clone());
        }
        (Some(201), Context::Channel(_)) => {
            print_colored_reply(&reply);
            let (thread_uuid, thread_timestamp) = match parse_created_payload(&reply) {
                Some((uuid, Some(timestamp))) => (uuid, timestamp),
                _ => return,
            };
            let args_parts = parse_quoted_segments(args);
            if args_parts.len() < 2 {
                return;
            }
            let user_uuid = io_manager.user_uuid.clone().unwrap_or_default();
            ClientLog::client_print_thread_created(thread_uuid, user_uuid, thread_timestamp, args_parts[0].clone(), args_parts[1].clone());
        }
        (Some(201), Context::Thread(_)) => {
            print_colored_reply(&reply);
            let (thread_uuid, reply_timestamp) = match parse_created_payload(&reply) {
                Some((uuid, Some(timestamp))) => (uuid, timestamp),
                _ => return,
            };
            let user_uuid = io_manager.user_uuid.clone().unwrap_or_default();
            let args_parts = parse_quoted_segments(args);
            let reply_body = args_parts.first().cloned().unwrap_or_else(|| args.trim().trim_matches('"').to_string());
            ClientLog::client_print_reply_created(thread_uuid, user_uuid, reply_timestamp, reply_body);
        }

        (Some(401), _) => {
            print_colored_reply(&reply);
            ClientLog::client_error_unauthorized();
        }
        (Some(404), _ctx) => {
            let parts: Vec<&str> = reply.splitn(5, ' ').collect();
            if let (Some(type_err), Some(entity_uuid)) = (parts.get(1), parts.get(4)) {
                let uuid = entity_uuid.trim_matches('"').trim_matches(|c| c == '\r' || c == '\n').trim_matches('"').to_string();
                match *type_err {
                    "TEAM" => {
                        ClientLog::client_error_unknown_team(uuid);
                        print_colored_reply(&reply);
                    }
                    "CHANNEL" => {
                        ClientLog::client_error_unknown_channel(uuid);
                        print_colored_reply(&reply);
                    }
                    "THREAD" => {
                        ClientLog::client_error_unknown_thread(uuid);
                        print_colored_reply(&reply);
                    }
                    _ => {
                        print_colored_reply(&reply);
                    }
                }
            } else {
                print_colored_reply(&reply);
            }
        }
        (Some(409), _) => {
            ClientLog::client_error_already_exist();
            print_colored_reply(&reply);
        }

        _ => {
            print_colored_reply(&reply);
        }
    }
}
