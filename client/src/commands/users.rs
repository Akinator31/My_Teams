use crate::client::io_manager::IoManager;
use crate::transport::{print_colored_reply, reply_code};
use libs::ClientLog;

fn get_user_info(line: &str) -> Option<(String, String, bool)> {
    let parts: Vec<&str> = line.split_whitespace().skip(1).collect();
    if parts.len() < 3 {
        return None;
    }
    let uuid = parts[0].to_string().trim_matches('"').to_string();
    let username = parts[1].to_string().trim_matches('"').to_string();
    let status: bool = parts[2].trim_matches('"').to_string().parse().ok()?;
    Some((uuid, username, status))
}

pub fn users(io_manager: &mut IoManager, _args: &str) {
    if let Err(e) = io_manager.write_line("USERS") {
        println!("Error occurred while writing to stream: {}", e);
    }
    loop {
        let reply = match io_manager.read_line() {
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
            print_colored_reply(&reply);
            ClientLog::client_error_unauthorized();
            break;
        }
        if (reply_code(&reply)) == Some(212) {
            print_colored_reply(&reply);
            for line in reply.lines() {
                if let Some((uuid, username, status)) = get_user_info(line) {
                    ClientLog::client_print_users(uuid, username, status.into());
                } else {
                    println!("Failed to parse user info from line: {}", line);
                }
            }
        } else {
            print_colored_reply(&reply);
            break;
        }
    }
}
