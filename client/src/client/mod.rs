pub mod io_manager;

use crate::client::io_manager::IoManager;
use crate::commands::commands::commands;
use crate::errors::errors::MyTeamsClientError;
use crate::events::events::events;
use std::env::args;

pub struct Client {
    pub server_address: String,
    pub server_port: u16,
    pub io_manager: Option<IoManager>,
}

fn get_server_infos() -> Result<(String, u16), MyTeamsClientError> {
    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        return Err(MyTeamsClientError::IncorrectArguments);
    }

    match args.as_slice() {
        [_prog, server_address, server_port] => {
            let server_port = server_port.parse::<u16>()?;

            Ok((server_address.clone(), server_port))
        }
        _ => Err(MyTeamsClientError::IncorrectArguments),
    }
}

impl Client {
    pub fn new() -> Result<Self, MyTeamsClientError> {
        let (server_address, server_port) = get_server_infos()?;

        Ok(Self {
            server_address,
            server_port,
            io_manager: None,
        })
    }

    pub fn connect(&mut self) -> Result<(), MyTeamsClientError> {
        let io_manager = IoManager::new(self.server_address.clone(), self.server_port)?;

        self.io_manager = Some(io_manager);
        Ok(())
    }

    pub fn poll(&mut self) -> Result<(), MyTeamsClientError> {
        match &mut self.io_manager {
            Some(io_manager) => io_manager.poll(),
            None => Ok(()),
        }
    }

    pub fn get_pending_server_message(&mut self) -> Option<String> {
        match &mut self.io_manager {
            Some(io_manager) => io_manager.get_pending_server_message(),
            None => None,
        }
    }

    pub fn get_pending_stdin_line(&mut self) -> Option<String> {
        match &mut self.io_manager {
            Some(io_manager) => io_manager.get_pending_stdin_line(),
            None => None,
        }
    }

    pub fn execute_stdin_command(&mut self, command_line: String) {
        let io_manager = match &mut self.io_manager {
            Some(io_manager) => io_manager,
            None => return,
        };

        let command_line = command_line.trim();
        if command_line.is_empty() {
            return;
        }

        let command_name = command_line
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_start_matches('/');

        let command_args: String = command_line
            .chars()
            .skip(command_name.len() + if command_line.starts_with('/') { 1 } else { 0 })
            .collect::<String>();

        if let Some(command_func) = commands().get(command_name) {
            command_func(io_manager, command_args.trim());
        } else {
            println!("Unknown command: {}", command_name);
        }
    }

    pub fn is_server_disconnected(&mut self) -> bool {
        match &mut self.io_manager {
            Some(io_manager) => io_manager.is_server_disconnected(),
            None => true,
        }
    }

    pub fn execute_server_event(&mut self, message: String) {
        if message.trim().starts_with("EVENT") {
            events(&message);
        }
    }
}
