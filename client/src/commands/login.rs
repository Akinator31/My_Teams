use crate::client::io_manager::IoManager;
use crate::transport::{print_colored_reply, reply_code};

fn parse_one_quoted_username(args: &str) -> Result<String, &'static str> {
    let s = args.trim();
    Ok(s.to_string())
}

fn extract_uuid_from_login_reply(line: &str) -> Option<String> {
    let trimmed = line.trim_end_matches(|c| c == '\r' || c == '\n');
    let after = trimmed.split("UUID:").nth(1)?;
    let uuid = after.trim().trim_matches('"').to_string();
    if uuid.is_empty() {
        None
    } else {
        Some(uuid)
    }
}

pub fn login(io_manager: &mut IoManager, args: &str) {
    let username = match parse_one_quoted_username(args) {
        Ok(u) => u,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    let line = format!("LOGIN {}", username);

    if let Err(e) = io_manager.write_line(&line) {
        println!("Failed to send login: {}", e);
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

    match reply_code(&reply) {
        Some(210) => {
            print_colored_reply(&reply);
            if let Some(uuid) = extract_uuid_from_login_reply(&reply) {
                io_manager.user_uuid = Some(uuid.clone());
                io_manager.user_name = Some(username.clone());
            } else {
                println!("Login succeeded but no UUID found in reply.");
            }
        }
        Some(_) => {
            print_colored_reply(&reply);
        }
        None => {
            println!("{}", reply.trim_end_matches(|c| c == '\r' || c == '\n'));
        }
    }
}
