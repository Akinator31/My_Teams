use crate::client::io_manager::IoManager;
use crate::transport::{print_colored_reply, reply_code};

pub fn set_context(io_manager: &mut IoManager, args: &str) {
    let cmd = format!("USE {}", args);

    if let Err(e) = io_manager.write_line(&cmd) {
        println!("Error occurred while writing to stream: {}", e);
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
        Some(250) => {
            print_colored_reply(&reply);
            println!("Switched context successfully.");
        }
        _ => {
            print_colored_reply(&reply);
        }
    }
}
