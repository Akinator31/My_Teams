use crate::errors::errors::MyTeamsClientError;
use crate::utils::select::{FdSet, Timeval, fd_select};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::os::unix::io::AsRawFd;

const STDIN_FD: i32 = 0;

unsafe extern "C" {
    fn read(fd: i32, buf: *mut u8, count: usize) -> isize;
}
pub struct ContextChannel {
    pub team: String,
    pub channel: String,
}
pub struct ContextThread {
    pub team: String,
    pub channel: String,
    pub thread: String,
}

pub enum Context {
    None,
    Team(String),
    Channel(ContextChannel),
    Thread(ContextThread),
}

impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

pub struct IoManager {
    server_stream: TcpStream,

    server_incoming_buffer: Vec<u8>,
    stdin_incoming_buffer: Vec<u8>,

    pub user_uuid: Option<String>,
    pub user_name: Option<String>,
    pub context: Context,
}

impl IoManager {
    pub fn new(server_address: String, server_port: u16) -> Result<Self, MyTeamsClientError> {
        let server_stream = TcpStream::connect(format!("{}:{}", server_address, server_port))?;

        server_stream.set_nonblocking(true)?;

        Ok(Self {
            server_stream,
            server_incoming_buffer: Vec::new(),
            stdin_incoming_buffer: Vec::new(),
            user_uuid: None,
            user_name: None,
            context: Context::None,
        })
    }

    pub fn poll(&mut self) -> Result<(), MyTeamsClientError> {
        let mut read_fds = FdSet::new();
        read_fds.zero();

        let server_fd = self.server_stream.as_raw_fd();
        read_fds.set(STDIN_FD);
        read_fds.set(server_fd);

        let nfds = server_fd + 1;

        let mut timeout = Timeval {
            tv_sec: 0,
            tv_usec: 100000,
        };

        let result = fd_select(nfds, Some(&mut read_fds), None, None, Some(&mut timeout));

        if result < 0 {
            return Err(MyTeamsClientError::IoClientError(
                "Select failed".to_string(),
            ));
        }

        if result == 0 {
            return Ok(());
        }

        if read_fds.is_set(STDIN_FD) {
            self.read_stdin()?;
        }

        if read_fds.is_set(server_fd) {
            self.read_server()?;
        }

        Ok(())
    }

    fn read_stdin(&mut self) -> Result<(), MyTeamsClientError> {
        let mut buffer = [0u8; 1024];

        let bytes_read = unsafe { read(STDIN_FD, buffer.as_mut_ptr(), buffer.len()) };

        if bytes_read > 0 {
            self.stdin_incoming_buffer
                .extend_from_slice(&buffer[..bytes_read as usize]);
        }

        Ok(())
    }

    fn read_server(&mut self) -> Result<bool, MyTeamsClientError> {
        let mut buffer = [0u8; 1024];

        match self.server_stream.read(&mut buffer) {
            Ok(0) => Ok(true),
            Ok(n) => {
                self.server_incoming_buffer.extend_from_slice(&buffer[..n]);
                Ok(false)
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(false),
            Err(e) => Err(MyTeamsClientError::from(e)),
        }
    }

    pub fn get_pending_server_message(&mut self) -> Option<String> {
        if let Some(pos) = self
            .server_incoming_buffer
            .windows(2)
            .position(|w| w == b"\r\n")
        {
            let line: Vec<u8> = self.server_incoming_buffer.drain(..pos + 2).collect();
            Some(
                String::from_utf8_lossy(&line)
                    .parse()
                    .unwrap_or("FAILED TO EXTRACT".to_string()),
            )
        } else {
            None
        }
    }

    pub fn get_pending_stdin_line(&mut self) -> Option<String> {
        if let Some(pos) = self.stdin_incoming_buffer.iter().position(|&b| b == b'\n') {
            let line: Vec<u8> = self.stdin_incoming_buffer.drain(..pos + 1).collect();
            Some(String::from_utf8_lossy(&line).trim().to_string())
        } else {
            None
        }
    }

    pub fn write_line(&mut self, message: &str) -> Result<(), std::io::Error> {
        let formatted = format!("{}\r\n", message);
        let buf = formatted.as_bytes();
        let mut written = 0;

        while written < buf.len() {
            match self.server_stream.write(&buf[written..]) {
                Ok(n) => written += n,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        self.server_stream.flush()?;
        Ok(())
    }

    pub fn read_line(&mut self) -> Result<String, std::io::Error> {
        loop {
            if let Some(pos) = self
                .server_incoming_buffer
                .windows(2)
                .position(|w| w == b"\r\n")
            {
                let line: Vec<u8> = self.server_incoming_buffer.drain(..pos + 2).collect();
                return Ok(String::from_utf8_lossy(&line).to_string());
            }

            let mut buffer = [0u8; 1024];

            self.server_stream.set_nonblocking(false)?;
            let bytes_read = self.server_stream.read(&mut buffer)?;
            self.server_stream.set_nonblocking(true)?;

            if bytes_read == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::ConnectionAborted,
                    "Connection closed",
                ));
            }

            self.server_incoming_buffer
                .extend_from_slice(&buffer[..bytes_read]);
        }
    }

    pub fn is_server_disconnected(&mut self) -> bool {
        let mut buffer = [0u8; 1];

        match self.server_stream.peek(&mut buffer) {
            Ok(0) => true,
            Ok(_) => false,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => false,
            Err(_) => true,
        }
    }
}
