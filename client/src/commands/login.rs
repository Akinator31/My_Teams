use std::net::TcpStream;

use libs::ClientLog;

use crate::transport::{print_colored_reply, reply_code, read_line, write_line};

const MAX_NAME_LENGTH: usize = 32;

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

pub fn login(stream: &mut TcpStream, pending: &mut Vec<u8>, args: &str) {
    let username = match parse_one_quoted_username(args) {
        Ok(u) => u,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    let line = format!("LOGIN {}", username);

    if let Err(e) = write_line(stream, &line) {
        println!("Failed to send login: {}", e);
        return;
    }

    let reply = match read_line(stream, pending) {
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
