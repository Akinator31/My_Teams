use std::net::TcpStream;

pub struct Client {
    stream: TcpStream
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Self { stream }
    }
}
