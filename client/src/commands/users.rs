use crate::transport::read_line;
use crate::transport::reply_code;
use crate::transport::write_line;
use libs::ClientLog;
use std::net::TcpStream;

fn get_user_info(line: &str) -> Option<(String, String, i32)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }
    let uuid = parts[0].to_string();
    let username = parts[1].to_string();
    let status = parts[2].parse::<i32>().ok()?;
    Some((uuid, username, status))
}

pub fn users(stream: &mut TcpStream, buffer: &mut Vec<u8>, args: &str) {
    if let Err(e) = write_line(stream, "USERS") {
        eprintln!("Error occurred while writing to stream: {}", e);
    }
    let reply = match read_line(stream, buffer) {
        Ok(line) => line,
        Err(e) => {
            eprintln!("Error occurred while reading from stream: {}", e);
            return;
        }
    };
    match reply_code(&reply) {
        Some(212) => {
            for line in reply.lines().skip(1) {
                if let Some((uuid, username, status)) = get_user_info(line) {
                    println!("User: {} (UUID: {}, Status: {})", username, uuid, status);
                    ClientLog::client_print_users(uuid, username, status);
                } else {
                    eprintln!("Failed to parse user info from line: {}", line);
                }
            }
        }
        Some(200) => {
            println!("End of list.");
        }
        Some(code) => {
            eprintln!("Unexpected reply code: {}", code);
        }
        None => {
            eprintln!("Failed to parse reply code from: {}", reply);
        }
    }
}
