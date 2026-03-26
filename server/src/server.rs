use std::env::{args, Args};
use crate::clients::manager::MyTeamsClientManager;
use crate::errors::myteams_errors::MyTeamsServerError;

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

    pub fn handle_incoming_connections(&mut self) -> Result<(), MyTeamsServerError>{
        self.client_manager.connect_client()?;

        Ok(())
    }
}