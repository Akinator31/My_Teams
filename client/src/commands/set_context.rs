use crate::client::io_manager::IoManager;
use crate::client::io_manager::{Context, ContextChannel, ContextThread};
use crate::transport::{print_colored_reply, reply_code};

pub fn set_context(io_manager: &mut IoManager, args: &str) {
    let cmd = format!("USE {}", args);
    let context_to_set_on_sucess;
    let args = args.split_whitespace().collect::<Vec<&str>>();

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
            match args.as_slice() {
                [team_uuid] => {
                    context_to_set_on_sucess = Context::Team(team_uuid.to_string());
                }
                [team_uuid, channel_uuid] => {
                    context_to_set_on_sucess = Context::Channel(ContextChannel {
                        team: team_uuid.to_string(),
                        channel: channel_uuid.to_string(),
                    });
                }
                [team_uuid, channel_uuid, thread_uuid] => {
                    context_to_set_on_sucess = Context::Thread(ContextThread {
                        team: team_uuid.to_string(),
                        channel: channel_uuid.to_string(),
                        thread: thread_uuid.to_string(),
                    });
                }
                _ => {
                    context_to_set_on_sucess = Context::None;
                }
            }
            io_manager.context = context_to_set_on_sucess;
            print_colored_reply(&reply);
            println!("Switched context successfully.");
        }
        _ => {
            print_colored_reply(&reply);
        }
    }
}
