use crate::server::commands::create::create;
use crate::server::commands::help::help;
use crate::server::commands::info::info;
use crate::server::commands::list::list;
use crate::server::commands::login::login;
use crate::server::commands::logout::logout;
use crate::server::commands::messages::messages;
use crate::server::commands::send::send;
use crate::server::commands::set_context::set_context;
use crate::server::commands::subscribe::subscribe;
use crate::server::commands::subscribed::subscribed;
use crate::server::commands::unsubscribe::unsubscribe;
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
        cmd.insert("CREATE".to_string(), create);
        cmd.insert("SUBSCRIBE".to_string(), subscribe);
        cmd.insert("UNSUBSCRIBE".to_string(), unsubscribe);
        cmd.insert("SUBSCRIBED".to_string(), subscribed);
        cmd.insert("LIST".to_string(), list);
        cmd.insert("INFO".to_string(), info);
        cmd.insert("HELP".to_string(), help);

        cmd
    })
}
