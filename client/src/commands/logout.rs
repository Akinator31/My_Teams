use crate::client::io_manager::IoManager;
use crate::transport::{print_colored_reply, reply_code};
use libs::ClientLog;

pub fn logout(io_manager: &mut IoManager, _args: &str) {
    if let Err(e) = io_manager.write_line("LOGOUT") {
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
        Some(211) => {
            print_colored_reply(&reply);
            let uuid = io_manager.user_uuid.take().unwrap_or_default();
            let name = io_manager.user_name.take().unwrap_or_default();
            ClientLog::client_event_logged_out(uuid, name);
        }
        Some(403) => {
            print_colored_reply(&reply);
            ClientLog::client_error_unauthorized();
        }
        _ => {
            print_colored_reply(&reply);
        }
    }
}
