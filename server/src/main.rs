mod errors;

use crate::errors::myteams_errors::MyTeamsServerError;
use std::env::{args, Args};
use std::net::TcpListener;
use std::process::exit;

fn get_server_port(args: Args) -> Result<String, MyTeamsServerError> {
    if args.len() != 2 {
        return Err(MyTeamsServerError::IncorrectArguments);
    }

    match args.last() {
        Some(port) => Ok(port),
        None => Err(MyTeamsServerError::IncorrectArguments),
    }
}

fn main() {
    let server_port = match get_server_port(args()) {
        Ok(port) => port,
        Err(e) => {
            println!("{}", e.to_string());
            exit(84);
        }
    };

    let listener = match TcpListener::bind(format!("0.0.0.0:{}", server_port)) {
        Ok(listener) => listener,
        Err(e) => {
            println!("{}", e.to_string());
            exit(84);
        }
    };

    match listener.set_nonblocking(true) {
        Ok(..) => {}
        Err(e) => {
            println!("{}", e.to_string());
            exit(84);
        }
    }
}
