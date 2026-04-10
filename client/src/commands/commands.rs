use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::OnceLock;

use crate::commands::login::login;
use crate::commands::user::user;
use crate::commands::mt_use::mt_use;
use crate::commands::send::send;
use crate::commands::users::users;

type ClientCommandType = HashMap<String, fn(&mut TcpStream, &mut Vec<u8>, &str)>;

static COMMANDS: OnceLock<ClientCommandType> = OnceLock::new();

pub fn commands() -> &'static ClientCommandType {
    COMMANDS.get_or_init(|| {
        let mut cmds: ClientCommandType = HashMap::new();

        cmds.insert("login".to_string(), login);
        cmds.insert("users".to_string(), users);
        cmds.insert("user".to_string(), user);
        cmds.insert("use".to_string(), mt_use);
        cmds.insert("send".to_string(), send);
        cmds
    })
}
