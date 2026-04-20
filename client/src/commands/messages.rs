use crate::client::io_manager::IoManager;
use crate::transport::{print_colored_reply, reply_code};
use libs::ClientLog;
use crate::utils::parsing::parse_quoted_segments;

fn parse_and_validate_uuid(args: &str) -> Option<String> {
    let uuid = args.trim();

    let unquoted_uuid = uuid.trim_matches('"');
    if unquoted_uuid.len() != 36 {
        return None;
    }
    if unquoted_uuid.chars().filter(|&c| c == '-').count() != 4 {
        return None;
    }
    Some(uuid.to_string())
}

pub fn messages(io_manager: &mut IoManager, args: &str) {
    let uuid: Option<String> = parse_and_validate_uuid(args);
    if uuid.is_none() {
        println!("Invalid UUID format. Please provide a valid UUID.");
        return;
    }
    let line: String = format!("MESSAGES {}", uuid.unwrap().to_string());
    if let Err(e) = io_manager.write_line(&*line) {
        println!("Error occurred while writing to stream: {}", e);
        return;
    }

    loop {
        let reply: String = match io_manager.read_line() {
            Ok(line) => line,
            Err(e) => {
                println!("Error occurred while reading from stream: {}", e);
                break;
            }
        };

        if reply.trim().starts_with("EVENT") {
            crate::events::events::events(&reply);
            continue;
        }

        if (reply_code(&reply)) == Some(200) {
            print_colored_reply(&reply);
            break;
        }
        if (reply_code(&reply)) == Some(401) {
            ClientLog::client_error_unauthorized();
            print_colored_reply(&reply);
            break;
        }
        if (reply_code(&reply)) == Some(404) {
             let parts: Vec<&str> = reply.splitn(5, ' ').collect();
             if let Some(entity_uuid) = parts.get(4) {
                 let uuid = entity_uuid.trim_matches('"').trim_matches(|c| c == '\r' || c == '\n').trim_matches('"').to_string();
                 ClientLog::client_error_unknown_user(uuid);
             }
             print_colored_reply(&reply);
             break;
        }
        if (reply_code(&reply)) == Some(221) {
            print_colored_reply(&reply);
            let parts: Vec<&str> = reply.splitn(4, ' ').collect();
            if parts.len() == 4 {
                let sender_uuid: String = parts[1].trim_matches('"').to_string();
                let timestamp_string = &parse_quoted_segments(parts[2])[0];
                let timestamp: i64 = timestamp_string.parse().unwrap_or(0);
                let body: String = parts[3].trim().trim_matches('"').trim_matches(|c| c == '\r' || c == '\n').trim_matches('"').to_string();
                if sender_uuid.is_empty() || body.is_empty() {
                    println!("Received an invalid message format.");
                    continue;
                }
                ClientLog::client_private_message_print_messages(sender_uuid, timestamp, body);
            }
        } else {
            print_colored_reply(&reply);
            break;
        }
    }
}
