use crate::client::io_manager::IoManager;
use crate::transport::{print_colored_reply, reply_code};
use libs::ClientLog;

pub fn unsubscribe(io_manager: &mut IoManager, args: &str) {
    let cmd = format!("UNSUBSCRIBE {}", args);

    if let Err(e) = io_manager.write_line(&cmd) {
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
        Some(200) => {
            print_colored_reply(&reply);
            let Some(uuid) = io_manager.user_uuid.as_ref() else {
                ClientLog::client_print_unsubscribed(String::new(), args.trim().to_string());
                return;
            };

            ClientLog::client_print_unsubscribed(uuid.clone(), args.trim().to_string());
        }
        Some(401) => {
            print_colored_reply(&reply);
            ClientLog::client_error_unauthorized();
        }
        Some(404) => {
            print_colored_reply(&reply);
            let parts: Vec<&str> = reply.splitn(5, ' ').collect();
            let uuid = if let Some(entity_uuid) = parts.get(4) {
                 entity_uuid.trim_matches('"').trim_matches(|c| c == '\r' || c == '\n').trim_matches('"').to_string()
            } else {
                args.trim().trim_matches('"').to_string()
            };
            ClientLog::client_error_unknown_team(uuid);
        }
        _ => {
            print_colored_reply(&reply);
        }
    }
}
