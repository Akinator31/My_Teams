use crate::server::commands::login::login;
use crate::server::commands::logout::logout;
use crate::server::commands::messages::messages;
use crate::server::commands::send::send;
use crate::server::commands::set_context::set_context;
use crate::server::commands::user::user;
use crate::server::commands::users::users;
use crate::server::server::MyTeamsServer;
use std::collections::HashMap;
use std::sync::OnceLock;

type CommandType = HashMap<String, fn(&mut MyTeamsServer, usize, String) -> bool>;

static COMMANDS: OnceLock<CommandType> = OnceLock::new();

pub fn commands() -> &'static CommandType {
    COMMANDS.get_or_init(|| {
        let mut cmd: CommandType = HashMap::new();
        cmd.insert("LOGIN".to_string(), login);
        cmd.insert("LOGOUT".to_string(), logout);
        cmd.insert("SEND".to_string(), send);
        cmd.insert("MESSAGES".to_string(), messages);
        cmd.insert("USERS".to_string(), users);
        cmd.insert("USER".to_string(), user);
        cmd.insert("USE".to_string(), set_context);

        cmd
    })
}
