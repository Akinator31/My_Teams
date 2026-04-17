use std::collections::HashMap;
use std::sync::OnceLock;

use crate::client::io_manager::IoManager;
use crate::commands::create::create;
use crate::commands::info::info;
use crate::commands::list_cmd::list_cmd;
use crate::commands::login::login;
use crate::commands::logout::logout;
use crate::commands::messages::messages;
use crate::commands::send::send;
use crate::commands::set_context::set_context;
use crate::commands::subscribe::subscribe;
use crate::commands::unsubscribe::unsubscribe;
use crate::commands::user::user;
use crate::commands::users::users;

type ClientCommandType = HashMap<String, fn(&mut IoManager, &str)>;

static COMMANDS: OnceLock<ClientCommandType> = OnceLock::new();

pub fn commands() -> &'static ClientCommandType {
    COMMANDS.get_or_init(|| {
        let mut cmds: ClientCommandType = HashMap::new();

        cmds.insert("login".to_string(), login);
        cmds.insert("logout".to_string(), logout);
        cmds.insert("users".to_string(), users);
        cmds.insert("user".to_string(), user);
        cmds.insert("use".to_string(), set_context);
        cmds.insert("send".to_string(), send);
        cmds.insert("subscribe".to_string(), subscribe);
        cmds.insert("unsubscribe".to_string(), unsubscribe);
        cmds.insert("info".to_string(), info);
        cmds.insert("messages".to_string(), messages);
        cmds.insert("create".to_string(), create);
        cmds.insert("list".to_string(), list_cmd);
        cmds
    })
}
