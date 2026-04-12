use std::fmt;
use std::fmt::Formatter;
use std::io::Error;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum MyTeamsClientError {
    IncorrectArguments,
    IncorrectServerPort,
    IoClientError(String),
}

impl fmt::Display for MyTeamsClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MyTeamsClientError::IncorrectArguments => {
                write!(f, "Wrong number of arguments!")
            }
            MyTeamsClientError::IncorrectServerPort => {
                write!(f, "Incorrect server port!")
            }
            MyTeamsClientError::IoClientError(e) => {
                write!(f, "IO Error on client : {}", e)
            }
        }
    }
}

impl From<ParseIntError> for MyTeamsClientError {
    fn from(_value: ParseIntError) -> Self {
        MyTeamsClientError::IncorrectServerPort
    }
}

impl From<Error> for MyTeamsClientError {
    fn from(value: Error) -> Self {
        MyTeamsClientError::IoClientError(value.to_string())
    }
}

impl std::error::Error for MyTeamsClientError {}
