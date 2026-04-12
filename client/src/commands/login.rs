use crate::client::io_manager::IoManager;
use crate::transport::{print_colored_reply, reply_code};
use libs::ClientLog;

fn parse_one_quoted_username(args: &str) -> Result<String, &'static str> {
    let s = args.trim();
    Ok(s.to_string())
}

fn extract_uuid_from_login_reply(line: &str) -> Option<String> {
    let trimmed = line.trim_end_matches(|c| c == '\r' || c == '\n');
    let after = trimmed.split("UUID:").nth(1)?;
    let uuid = after.trim();
    if uuid.is_empty() {
        None
    } else {
        Some(uuid.to_string())
    }
}

pub fn login(io_manager: &mut IoManager, args: &str) {
    let username = match parse_one_quoted_username(args) {
        Ok(u) => u,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    let line = format!("LOGIN {}", username);

    if let Err(e) = io_manager.write_line(&line) {
        println!("Failed to send login: {}", e);
        return;
    }

    let reply = match io_manager.read_line() {
        Ok(l) => l,
        Err(e) => {
            println!("Failed to read server reply: {}", e);
            return;
        }
    };

    match reply_code(&reply) {
        Some(210) => {
            print_colored_reply(&reply);
            if let Some(uuid) = extract_uuid_from_login_reply(&reply) {
                ClientLog::client_event_logged_in(uuid, username);
            } else {
                println!("Login succeeded but no UUID found in reply.");
            }
        }
        Some(_) => {
            print_colored_reply(&reply);
        }
        None => {
            println!("{}", reply.trim_end_matches(|c| c == '\r' || c == '\n'));
        }
    }
}
