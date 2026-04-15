use std::fmt;
use std::fmt::Formatter;
use std::io::Error;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum MyTeamsServerError {
    IncorrectArguments,
    ServerPortParseError,
    IoError,
    ClientConnectionError,
    AlreadyExist,
    TeamNotFound(String),
    ChannelNotFound(String),
    ThreadNotFound(String),
}

impl fmt::Display for MyTeamsServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MyTeamsServerError::IncorrectArguments => {
                write!(
                    f,
                    "Wrong number of arguments! You must provide the port the server should listen on! See help command for more details."
                )
            }
            MyTeamsServerError::ServerPortParseError => {
                write!(
                    f,
                    "Failed to parse the given server port! It must be a value from 1 to 65 535!"
                )
            }
            MyTeamsServerError::IoError => {
                write!(
                    f,
                    "IO Error!"
                )
            }
            MyTeamsServerError::ClientConnectionError => {
                write!(
                    f,
                    "An error occurred during the client connection!"
                )
            }
            MyTeamsServerError::AlreadyExist => {
                write!(
                    f,
                    "The resource already exists!"
                )
            }
            _ => {
                write!(
                    f,
                    "An error occurred!"
                )
            }
        }
    }
}

impl From<ParseIntError> for MyTeamsServerError {
    fn from(_value: ParseIntError) -> Self {
        MyTeamsServerError::ServerPortParseError
    }
}

impl From<Error> for MyTeamsServerError {
    fn from(_value: Error) -> Self {
        MyTeamsServerError::IoError
    }
}

impl std::error::Error for MyTeamsServerError {}
