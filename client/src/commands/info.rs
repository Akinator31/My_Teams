use crate::client::io_manager::IoManager;
use crate::transport::{print_colored_reply, reply_code};
use libs::ClientLog;

fn get_user_info(line: &str) -> Option<(String, String, i32)> {
    let parts: Vec<&str> = line.split_whitespace().skip(1).collect();
    if parts.len() < 3 {
        return None;
    }
    let uuid = parts[0].to_string().trim_matches('"').to_string();
    let username = parts[1].to_string().trim_matches('"').to_string();
    let status: i32 = parts[2].trim_matches('"').to_string().parse().ok()?;
    Some((uuid, username, status))
}

fn format_for_channel_and_team(line: &str) -> Option<(String, String, String)> {
    let parts: Vec<&str> = line.split_whitespace().skip(1).collect();
    if parts.len() < 3 {
        return None;
    }
    let uuid = parts[0].to_string().trim_matches('"').to_string();
    let name = parts[1].to_string().trim_matches('"').to_string();
    let description = parts[2..].join(" ").trim_matches('"').to_string();
    Some((uuid, name, description))
}

fn format_thread_info(line: &str) -> Option<(String, String, String, String, i64)> {
    let parts: Vec<&str> = line.split_whitespace().skip(1).collect();
    let uuid = parts[0].trim_matches('"').to_string();
    let title = parts[1].trim_matches('"').to_string();
    let msg = parts[2].trim_matches('"').to_string();
    let creator_uuid = parts[3].trim_matches('"').to_string();
    let timestamp = parts[4].trim_matches('"').parse::<i64>().ok()?;
    Some((uuid, creator_uuid, title, msg, timestamp))
}

pub fn info(io_manager: &mut IoManager, args: &str) {
    let cmd = format!("INFO {}", args);

    if let Err(e) = io_manager.write_line(&cmd) {
        println!("Error occurred while writing to stream: {}", e);
        return;
    }

    let reply = match io_manager.read_line() {
        Ok(line) => line,
        Err(e) => {
            println!("Error occurred while reading from stream: {}", e);
            return;
        }
    };

    let code = reply_code(&reply);

    match code {
        Some(213) => {
            let user_info = get_user_info(&reply);
            if let Some((uuid, username, status)) = user_info {
                ClientLog::client_print_user(uuid, username, status);
            }
        }
        Some(240) => {
            let team_info = format_for_channel_and_team(&reply);
            if let Some((uuid, name, description)) = team_info {
                ClientLog::client_print_team(uuid, name, description);
            }
        }
        Some(241) => {
            let channel_info = format_for_channel_and_team(&reply);
            if let Some((uuid, name, description)) = channel_info {
                ClientLog::client_print_channel(uuid, name, description);
            }
        }
        Some(242) => {
            let thread_info = format_thread_info(&reply);
            if let Some((uuid, creator_uuid, title, msg, timestamp)) = thread_info {
                ClientLog::client_print_thread(uuid, creator_uuid, timestamp, title, msg);
            }
        }
        Some(401) => {
            ClientLog::client_error_unauthorized();
        }
        _ => {
            print_colored_reply(&reply);
        }
    }
}
