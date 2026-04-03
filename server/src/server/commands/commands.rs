use std::collections::HashMap;
use std::sync::OnceLock;
use crate::server::commands::login::login;
use crate::server::commands::logout::logout;
use crate::server::commands::send::send;
use crate::server::server::MyTeamsServer;

type CommandType = HashMap<String, fn(&mut MyTeamsServer, usize, String) -> bool>;

static COMMANDS: OnceLock<CommandType> = OnceLock::new();

pub fn commands() -> &'static CommandType {
    COMMANDS.get_or_init(|| {
        let mut cmd: CommandType = HashMap::new();
        cmd.insert("LOGIN".to_string(), login);
        cmd.insert("LOGOUT".to_string(), logout);
        cmd.insert("SEND".to_string(), send);

        cmd
    })
}


