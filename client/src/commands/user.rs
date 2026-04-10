use crate::transport::print_colored_reply;
use crate::transport::read_line;
use crate::transport::reply_code;
use crate::transport::write_line;
use libs::ClientLog;
use std::net::TcpStream;

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

pub fn user(stream: &mut TcpStream, buffer: &mut Vec<u8>, args: &str) {
    if let Err(e) = write_line(stream, format!("USER {}", args).as_str()) {
        println!("Error occurred while writing to stream: {}", e);
    }

    let reply = match read_line(stream, buffer) {
        Ok(line) => line,
        Err(e) => {
            println!("Error occurred while reading from stream: {}", e);
            return;
        }
    };

    let code = reply_code(&reply);

    match code {
        Some(213) => {
            print_colored_reply(&reply);
            if let Some((uuid, username, status)) = get_user_info(&reply) {
                ClientLog::client_print_user(uuid, username, status.into());
            } else {
                println!("Failed to parse user info from reply: {}", reply);
            }
        }
        Some(404) => {
            print_colored_reply(&reply);
            let uuid = args.trim().to_string();
            ClientLog::client_error_unknown_user(uuid);
        }
        _ => print_colored_reply(&reply),
    }
}
