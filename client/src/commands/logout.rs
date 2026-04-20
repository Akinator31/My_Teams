use crate::client::io_manager::IoManager;
use crate::transport::{print_colored_reply, reply_code};
use libs::ClientLog;

pub fn logout(io_manager: &mut IoManager, _args: &str) {
    if let Err(e) = io_manager.write_line("LOGOUT") {
        println!("Error occurred while writing to stream: {}", e);
        return;
    }

    let reply = loop {
        let line = match io_manager.read_line() {
            Ok(line) => line,
            Err(e) => {
                println!("Error occurred while reading from stream: {}", e);
                return;
            }
        };
        if line.trim().starts_with("EVENT") {
            crate::events::events::events(&line);
            continue;
        }
        break line;
    };

    let code = reply_code(&reply);

    match code {
        Some(211) => {
            print_colored_reply(&reply);
            let _uuid = io_manager.user_uuid.take().unwrap_or_default().trim_matches('"').to_string();
            let _name = io_manager.user_name.take().unwrap_or_default().trim_matches('"').to_string();
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
