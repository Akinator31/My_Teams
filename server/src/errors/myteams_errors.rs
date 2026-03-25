use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum MyTeamsServerError {
    IncorrectArguments,
    FailedToBind,
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
            MyTeamsServerError::FailedToBind => {
                write!(f, "Failed to bind TCP server port!")
            }
        }
    }
}

impl std::error::Error for MyTeamsServerError {}
