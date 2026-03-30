use std::collections::HashMap;
use std::sync::OnceLock;
use crate::commands::login::login;

type ClientCommandType = HashMap<String, fn () -> ()>;

static COMMANDS: OnceLock<HashMap<String, fn () -> ()>> = OnceLock::new();

pub fn commands() -> &'static ClientCommandType {
    COMMANDS.get_or_init(|| {
        let mut cmds: ClientCommandType = HashMap::new();

        cmds.insert("login".to_string(), login);
        cmds
    })
}