use std::io::Read;
use std::io::ErrorKind::WouldBlock;
use std::net::TcpStream;
use crate::clients::client::ClientState::ToBeDisconnected;
use crate::errors::myteams_errors::MyTeamsServerError;

#[derive(Debug, PartialEq)]
pub enum ClientState {
    None,
    ToBeDisconnected
}

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
    incoming_data_buffer: Vec<u8>,
    pub state: ClientState,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        let incoming_data_buffer = Vec::new();
        let state = ClientState::None;

        match stream.set_nonblocking(true) {
            Ok(_) => Self { stream, incoming_data_buffer, state },
            Err(e) => {
                println!("Failed to set client socket non blocking: {}", e.to_string());
                Self {stream, incoming_data_buffer, state }
            }
        }
    }

    pub fn read(&mut self) -> Result<(), MyTeamsServerError> {
        let mut buffer = [0; 1024];

        loop {
            match self.stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Connexion closed from {:?}", self.stream);
                    self.state = ToBeDisconnected;
                    return Ok(());
                }
                Ok(o) => {
                    self.incoming_data_buffer.extend_from_slice(&buffer[..o]);
                }
                Err(e) if e.kind() == WouldBlock => {
                        return Ok(());
                }
                Err(e) => {
                    return Err(MyTeamsServerError::from(e));
                }
            }
        }
    }
}
