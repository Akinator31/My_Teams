use std::net::TcpStream;
use std::io::{Write};

pub fn write_line(stream: &mut TcpStream, line_without_crlf: &str) -> std::io::Result<()> {
    stream.write_all(line_without_crlf.as_bytes())?;
    stream.write_all(b"\r\n")?;
    Ok(())
}
