use std::env::{args, Args};
use crate::clients::client::ReplyCode::BadRequest;
use crate::clients::manager::MyTeamsClientManager;
use crate::errors::myteams_errors::MyTeamsServerError;
use crate::server::commands::commands::commands;

fn get_server_port(args: Args) -> Result<String, MyTeamsServerError> {
    if args.len() != 2 {
        return Err(MyTeamsServerError::IncorrectArguments);
    }

    match args.last() {
        Some(port) => Ok(port),
        None => Err(MyTeamsServerError::IncorrectArguments),
    }
}

pub struct MyTeamsServer {
    pub client_manager: MyTeamsClientManager,
}

impl MyTeamsServer {
    pub fn new() -> Result<Self, MyTeamsServerError> {
        let port = get_server_port(args())?.parse::<u16>()?;
        let client_manager = MyTeamsClientManager::new(port)?;

        Ok(Self { client_manager })
    }

    pub fn execute_clients_pending_command(&mut self) {
        for client in &mut self.client_manager.clients {
            if let Some(command) = client.get_pending_command() {
                let mut command_name = command.split(" ").next().unwrap_or("");

                if command_name == command {
                    command_name = command.split("\r\n").next().unwrap_or("");
                }

                if let Some(command_func) = commands().get(&command_name.to_string()) {
                    let command_args: String = command.chars().skip(command_name.len()).collect::<String>();
                    if !command_func(client, command_args) {
                        client.write(BadRequest);
                    }
                } else {
                    client.write(BadRequest);
                }
            }
        }
    }
}