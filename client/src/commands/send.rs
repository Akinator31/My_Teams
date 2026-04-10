use crate::transport::print_colored_reply;
use crate::transport::read_line;
use crate::transport::reply_code;
use crate::transport::write_line;
use libs::ClientLog;
use std::net::TcpStream;

fn get_message_info(line: &str) -> (String, String) {
    let parts: Vec<&str> = line.split_whitespace().skip(1).collect();
    if parts.len() < 2 {
        return (String::new(), String::new());
    }
    let uuid = parts[0].to_string().trim_matches('"').to_string();
    let message = parts[1..].join(" ").trim_matches('"').to_string();
    (uuid, message)
}

pub fn send(stream: &mut TcpStream, buffer: &mut Vec<u8>, args: &str) {
    let formatted_args = format!("SEND {}", args.trim());

    if let Err(e) = write_line(stream, &formatted_args) {
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
    let message_info = get_message_info(&reply);

    match code {
        Some(220) => {
            print_colored_reply(&reply);
            println!("Message sent successfully.");
        }
        Some(404) => {
            print_colored_reply(&reply);
            ClientLog::client_error_unknown_user(message_info.0);
        }
        Some(401) => {
            print_colored_reply(&reply);
            ClientLog::client_error_unauthorized();
        }
        _ => {
            print_colored_reply(&reply);
        }
    }
}
