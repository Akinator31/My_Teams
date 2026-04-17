use crate::client::io_manager::{Context, IoManager};
use crate::transport::{print_colored_reply, reply_code};
use crate::utils::parsing::parse_quoted_segments;
use libs::ClientLog;

fn list_context_label(context: &Context) -> &'static str {
    match context {
        Context::None => "NONE",
        Context::Team(_) => "TEAM",
        Context::Channel(_) => "CHANNEL",
        Context::Thread(_) => "THREAD",
    }
}

pub fn list_cmd(io_manager: &mut IoManager, _args: &str) {
    let line: String = format!("LIST {}", list_context_label(&io_manager.context));
    if let Err(e) = io_manager.write_line(&*line) {
        println!("Error occurred while writing to stream: {}", e);
        return;
    }

    loop {
        let reply = match io_manager.read_line() {
            Ok(line) => line,
            Err(e) => {
                println!("Error while reading from stream: {}", e);
                return;
            }
        };

        match reply_code(&reply) {
            Some(230) => {
                print_colored_reply(&reply);
                let parts = parse_quoted_segments(&reply);
                if parts.len() >= 3 {
                    ClientLog::client_print_teams(
                        parts[0].clone(),
                        parts[1].clone(),
                        parts[2].clone(),
                    );
                }
            }
            Some(233) => {
                print_colored_reply(&reply);
                let parts = parse_quoted_segments(&reply);
                if parts.len() >= 3 {
                    ClientLog::client_team_print_channels(
                        parts[0].clone(),
                        parts[1].clone(),
                        parts[2].clone(),
                    );
                }
            }
            Some(234) => {
                print_colored_reply(&reply);
                let parts = parse_quoted_segments(&reply);
                if parts.len() >= 4 {
                    let timestamp = parts[3].parse::<i64>().unwrap_or(0);
                    ClientLog::client_channel_print_threads(
                        parts[0].clone(),
                        parts[2].clone(),
                        timestamp,
                        parts[1].clone(),
                        String::new(),
                    );
                }
            }
            Some(235) => {
                print_colored_reply(&reply);
                let parts = parse_quoted_segments(&reply);
                if parts.len() >= 4 {
                    let timestamp = parts[3].parse::<i64>().unwrap_or(0);
                    ClientLog::client_thread_print_replies(
                        parts[0].clone(),
                        parts[2].clone(),
                        timestamp,
                        parts[1].clone(),
                    );
                }
            }
            Some(401) => {
                ClientLog::client_error_unauthorized();
                print_colored_reply(&reply);
                break;
            }
            Some(200) => {
                print_colored_reply(&reply);
                break;
            }
            _ => {
                print_colored_reply(&reply);
                break;
            }
        }
    }
}
