use crate::client::io_manager::IoManager;
use crate::transport::{print_colored_reply, reply_code};
use libs::ClientLog;

pub fn unsubscribe(io_manager: &mut IoManager, args: &str) {
    let cmd = format!("UNSUBSCRIBE {}", args);

    if let Err(e) = io_manager.write_line(&cmd) {
        println!("Error occurred while writing to stream: {}", e);
        return;
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
        Some(200) => {
            print_colored_reply(&reply);
            ClientLog::client_print_unsubscribed(String::new(), args.trim().to_string());
        }
        Some(404) => {
            print_colored_reply(&reply);
            ClientLog::client_error_unknown_team(args.trim().to_string());
        }
        _ => {
            print_colored_reply(&reply);
        }
    }
}
