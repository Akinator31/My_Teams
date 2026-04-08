use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;
use std::io::ErrorKind::WouldBlock;
use std::time::Duration;
use crate::errors::errors::MyTeamsServerError;

pub fn write_line(stream: &mut TcpStream, line_without_crlf: &str) -> std::io::Result<()> {
    stream.write_all(line_without_crlf.as_bytes())?;
    stream.write_all(b"\r\n")?;
    Ok(())
}

pub fn reply_code(line: &str) -> Option<u16> {
    line.get(0..3)?.parse().ok()
}

pub fn read_line(stream: &mut TcpStream, pending: &mut Vec<u8>) -> std::io::Result<String> {
    let mut byte = [0u8; 1];
    loop {
        match stream.read(&mut byte) {
            Ok(0) => {
                return Err(std::io::Error::other(MyTeamsServerError::UnexpectedEof.to_string()));
            }
            Ok(_) => {
                pending.push(byte[0]);
                if pending.ends_with(b"\r\n") {
                    pending.truncate(pending.len() - 2);
                    return Ok(String::from_utf8_lossy(pending).into_owned());
                }
            }
            Err(e) if e.kind() == WouldBlock => {
                thread::sleep(Duration::from_millis(5));
            }
            Err(e) => return Err(e),
        }
    }
}