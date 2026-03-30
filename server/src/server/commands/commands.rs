use std::collections::HashMap;
use std::sync::OnceLock;
use crate::clients::client::Client;
use crate::server::commands::login::login;
use crate::server::data::MyTeamsServerData;

type CommandType = HashMap<String, fn(&mut MyTeamsServerData, &mut Client, String) -> bool>;

static COMMANDS: OnceLock<CommandType> = OnceLock::new();

pub fn commands() -> &'static CommandType {
    COMMANDS.get_or_init(|| {
        let mut cmd: CommandType = HashMap::new();
        cmd.insert("LOGIN".to_string(), login);

        cmd
    })
}


