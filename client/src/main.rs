mod commands;
mod errors;

use std::env::args;
use std::io::{BufRead};
use std::net::TcpStream;
use std::process::exit;
use crate::commands::commands::commands;

fn usage(binName: String) {
    println!("USAGE: {} <ip> <port>", binName);
}

fn client(mut stream: TcpStream) {
    stream.set_nonblocking(true);
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line_result) = lines.next() {
        match line_result {
            Ok(line) => {
                if !line.starts_with("/") {
                    println!("Incorrect command!");
                    continue
                }
                let command = line.split(" ").next().unwrap_or("");
                let command_trimmed = command.strip_prefix("/").unwrap();

                if let Some(command_trimmed) = commands().get(command_trimmed) {
                    command_trimmed();
                } else {
                    println!("Unknow command!");
                }
            }
            Err(e) => {
                eprintln!("An error occured reading stdin : {}", e);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();

    // libs::ClientLog::client_event_logged_in("Holamos".to_string(), "Holamos".to_string());
    if args.len() != 3 {
        usage(args[0].clone());
        exit(84);
    }
    let ip: String = args[1].clone();
    let port: u16 = args[2].clone().parse().unwrap();
    let address = format!("{}:{}", ip, port);

    let mut stream = TcpStream::connect(address)?;
    client(stream); // parcontre ça stop le serveur quand je stop le client wtf (Error: IoError)
    Ok(())
}
