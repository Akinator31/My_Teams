use crate::clients::client::ErrorCode::BadRequest;
use crate::clients::manager::MyTeamsClientManager;
use crate::errors::myteams_errors::MyTeamsServerError;
use crate::server::commands::commands::commands;
use crate::server::data::MyTeamsServerData;
use std::env::{args, Args};

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

    pub data: MyTeamsServerData,
}

impl MyTeamsServer {
    pub fn new() -> Result<Self, MyTeamsServerError> {
        let port = get_server_port(args())?.parse::<u16>()?;
        let client_manager = MyTeamsClientManager::new(port)?;

        Ok(Self {
            client_manager,
            data: MyTeamsServerData::new(),
        })
    }

    pub fn execute_clients_pending_command(&mut self) {
        let pending: Vec<(usize, String)> = self
            .client_manager
            .clients
            .iter_mut()
            .enumerate()
            .filter_map(|(i, client)| client.get_pending_command().map(|cmd| (i, cmd)))
            .collect();

        for (index, command) in pending {
            let mut command_name = command.split(" ").next().unwrap_or("");

            if command_name == command {
                command_name = command.split("\r\n").next().unwrap_or("");
            }

            if let Some(command_func) = commands().get(&command_name.to_string()) {
                let command_args: String =
                    command.chars().skip(command_name.len()).collect::<String>();
                if !command_func(self, index, command_args) {
                    self.client_manager.clients[index].write(BadRequest);
                }
            } else {
                self.client_manager.clients[index].write(BadRequest);
            }
        }
    }
}
