use std::collections::HashMap;
use std::io::{Read, Write};
use std::io::ErrorKind::WouldBlock;
use std::net::TcpStream;
use std::sync::OnceLock;
use crate::clients::client::ClientState::ToBeDisconnected;
use crate::clients::client::ReplyCode::{BadRequest, Okay};
use crate::errors::myteams_errors::MyTeamsServerError;

#[derive(Debug, PartialEq)]
pub enum ClientState {
    None,
    ToBeDisconnected
}

#[derive(Debug, PartialEq, Eq)]
#[derive(Hash)]
pub enum ReplyCode {
    Okay,
    BadRequest,
}

static RESPONSES: OnceLock<HashMap<ReplyCode, String>> = OnceLock::new();

fn responses() -> &'static HashMap<ReplyCode, String> {
    RESPONSES.get_or_init(|| {
        let mut res = HashMap::new();

        res.insert(Okay, "200 Connected to MyTeams server\r\n".to_string());
        res.insert(BadRequest, "400 Bad request\r\n".to_string());
        res
    })
}

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
    incoming_data_buffer: Vec<u8>,
    pub state: ClientState,

    pub uuid: Option<String>
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        let incoming_data_buffer = Vec::new();
        let state = ClientState::None;

        match stream.set_nonblocking(true) {
            Ok(_) => Self { stream, incoming_data_buffer, state, uuid: None },
            Err(e) => {
                println!("Failed to set client socket non blocking: {}", e.to_string());
                Self {stream, incoming_data_buffer, state, uuid: None }
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

    pub fn write(&mut self, code: ReplyCode) {
        let response = responses()[&code].to_string();

        loop {
            match (self.stream).write_all(response.as_bytes()) {
                Ok(_) => break,
                Err(e) if e.kind() == WouldBlock => {
                    continue;
                }
                Err(e) => {
                    println!("An error occured during writing the resposes to tcpsocket: {}", e.to_string());
                    break;
                }
            }
        }
    }

    pub fn get_pending_command(&mut self) -> Option<String> {
        if let Some(pos) = self.incoming_data_buffer.windows(2).position(|w| w == b"\r\n") {
            let line: Vec<u8> = self.incoming_data_buffer.drain(..pos + 2).collect();
            Some(String::from_utf8_lossy(&line).parse().unwrap_or("FAILED TO EXTRACT ".to_string()))
        } else {
            None
        }
    }
}
