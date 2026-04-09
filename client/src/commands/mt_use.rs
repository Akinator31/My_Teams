use crate::transport::print_colored_reply;
use crate::transport::read_line;
use crate::transport::reply_code;
use crate::transport::write_line;
use std::net::TcpStream;

pub fn mt_use(stream: &mut TcpStream, buffer: &mut Vec<u8>, args: &str) {
    let cmd = format!("USE {}", args.trim());

    if let Err(e) = write_line(stream, &cmd) {
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
        Some(250) => {
            println!("Switched context successfully.");
        }
        _ => {
            print_colored_reply(&reply);
        }
    }
}
