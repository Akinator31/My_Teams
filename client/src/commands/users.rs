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

pub fn users(stream: &mut TcpStream, buffer: &mut Vec<u8>, args: &str) {
    if let Err(e) = write_line(stream, "USERS") {
        println!("Error occurred while writing to stream: {}", e);
    }
    loop {
        let reply = match read_line(stream, buffer) {
            Ok(line) => line,
            Err(e) => {
                println!("Error occurred while reading from stream: {}", e);
                break;
            }
        };
        if (reply_code(&reply)) == Some(200) {
            break;
        }
        if (reply_code(&reply)) == Some(212) {
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
