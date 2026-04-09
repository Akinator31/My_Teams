use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::OnceLock;

use crate::commands::login::login;
use crate::commands::mt_use::mt_use;
use crate::commands::users::users;

type ClientCommandType = HashMap<String, fn(&mut TcpStream, &mut Vec<u8>, &str)>;

static COMMANDS: OnceLock<ClientCommandType> = OnceLock::new();

pub fn commands() -> &'static ClientCommandType {
    COMMANDS.get_or_init(|| {
        let mut cmds: ClientCommandType = HashMap::new();

        cmds.insert("login".to_string(), login);
        cmds.insert("users".to_string(), users);
        cmds.insert("use".to_string(), mt_use);
        cmds
    })
}
