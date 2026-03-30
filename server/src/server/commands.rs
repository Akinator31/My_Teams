use std::collections::HashMap;
use std::sync::OnceLock;
use crate::clients::client::Client;
use crate::clients::client::ReplyCode::Okay;

type CommandType = HashMap<String, fn(&mut Client, String) -> ()>;

static COMMANDS: OnceLock<CommandType> = OnceLock::new();

fn login(client: &mut Client, command_args: String) -> () {
    println!("LOGIN COMMAND EXECUTED");
    client.write(Okay);
}

pub fn commands() -> &'static CommandType {
    COMMANDS.get_or_init(|| {
        let mut cmd: CommandType = HashMap::new();
        cmd.insert("LOGIN".to_string(), login);

        cmd
    })
}


