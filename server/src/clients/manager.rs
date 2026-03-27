use std::io::ErrorKind::WouldBlock;
use std::net::{TcpListener};
use crate::clients::client::Client;
use crate::errors::myteams_errors::MyTeamsServerError;
use crate::errors::myteams_errors::MyTeamsServerError::ClientConnectionError;

pub struct MyTeamsClientManager {
    pub clients: Vec<Client>,
    pub listener: TcpListener,
}

impl MyTeamsClientManager {
    pub fn new(port: u16) -> Result<Self, MyTeamsServerError> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        let clients = Vec::new();

        Ok(Self { clients, listener })
    }

    pub fn set_nonblocking(&self, value: bool) -> Result<&MyTeamsClientManager, MyTeamsServerError> {
        self.listener.set_nonblocking(value)?;

        Ok(self)
    }

    pub fn connect_client(&mut self) -> Result<(), MyTeamsServerError> {
        match self.listener.accept() {
            Ok((client_stream, _client_addr)) => {
                let new_client = Client::new(client_stream);
                println!("New client connected : {:?}", new_client);
                
                self.clients.push(new_client);

                Ok(())
            },
            Err(e) if e.kind() == WouldBlock => Ok(()),
            Err(_) => Err(ClientConnectionError)
        }
    }

    pub fn receive_clients_data(&mut self) -> Result<&MyTeamsClientManager, MyTeamsServerError> {
        for client in &mut self.clients {
            println!("{:?}", client);
            client.read()?;
        }

        Ok(self)
    }
}