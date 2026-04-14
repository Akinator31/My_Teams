use crate::client::io_manager::IoManager;
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
    match reply_code(&reply) {
        Some(201) => {
            print_colored_reply(&reply);
            return;
        }
        Some(_) => {
            print_colored_reply(&reply);
        }
        None => {
            println!("{}", reply.trim_end_matches(|c| c == '\r' || c == '\n'));
        }
    }
}
