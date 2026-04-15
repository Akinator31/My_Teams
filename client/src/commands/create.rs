use libs::ClientLog;
use crate::client::io_manager::{Context, IoManager};
use crate::transport::{print_colored_reply, reply_code};

pub fn create(io_manager: &mut IoManager, args: &str) {
    let line: String = format!("CREATE {}", args);
    if let Err(e) = io_manager.write_line(&*line) {
        println!("Error occurred while writing to stream: {}", e);
        return;
    }

    let reply = match io_manager.read_line() {
        Ok(line) => line,
        Err(e) => {
            println!("Error while reading from stream: {}", e);
            return;
        }
    };
    match (reply_code(&reply), &io_manager.context) {
        (Some(201), Context::None) => {
            print_colored_reply(&reply);
            let parts: Vec<&str> = reply.splitn(1, ' ').collect();
            let args_parts: Vec<&str> = reply.splitn(2, ' ').collect();
            let team_uuid = parts[0].trim_matches('"').to_string();
            let team_name = args_parts[0].trim_matches('"').to_string();
            let team_description = args_parts[1].trim_matches('"').to_string();
            ClientLog::client_print_team_created(team_uuid, team_name, team_description);
        }
        (Some(201), Context::Team(channel_uuid)) => {
            print_colored_reply(&reply);
            let args_parts: Vec<&str> = reply.splitn(2, ' ').collect();
            let channel_name = args_parts[0].trim_matches('"').to_string();
            let channel_description = args_parts[1].trim_matches('"').to_string();
            ClientLog::client_print_channel_created(channel_uuid.to_string(), channel_name, channel_description);
        }
        (Some(201), Context::Channel(_)) => {
            print_colored_reply(&reply);
            let parts: Vec<&str> = reply.splitn(2, ' ').collect();
            let args_parts: Vec<&str> = reply.splitn(2, ' ').collect();
            let thread_uuid = parts[0].trim_matches('"').to_string();
            let thread_timestamp: i64 = parts[1].parse().unwrap_or(0);
            let thread_title = args_parts[0].trim_matches('"').to_string();
            let thread_description = args_parts[1].trim_matches('"').to_string();
            ClientLog::client_print_thread_created(thread_uuid, io_manager.user_uuid.clone().unwrap().to_string(), thread_timestamp, thread_title, thread_description);
        }
        (Some(202), Context::Thread(ctx)) => {
            print_colored_reply(&reply);
            let parts: Vec<&str> = reply.splitn(2, ' ').collect();
            let thread_uuid = parts[0].trim_matches('"').to_string();
            let reply_timestamp: i64 = parts[1].parse().unwrap_or(0);
            ClientLog::client_print_reply_created(thread_uuid, io_manager.user_uuid.clone().unwrap().to_string(), reply_timestamp, args.to_string());
        }
        
        (Some(404), ctx) => {
            let parts: Vec<&str> = reply.splitn(5, ' ').collect();
            match (parts[1]) {
                "TEAM" => {
                    ClientLog::client_error_unknown_team(parts[4].to_string());
                    print_colored_reply(&reply);
                }
                "CHANNEL" => {
                    ClientLog::client_error_unknown_channel(parts[4].to_string());
                    print_colored_reply(&reply);
                }
                "THREAD" => {
                    ClientLog::client_error_unknown_thread(parts[4].to_string());
                    print_colored_reply(&reply);
                }
                _ => {
                    print_colored_reply(&reply);
                }
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
