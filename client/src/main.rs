mod commands;
mod errors;
mod transport;

use std::env::args;
use std::io::BufRead;
use std::net::TcpStream;
use std::process::exit;

use crate::commands::commands::commands;
use crate::transport::read_line;

fn usage(bin_name: String) {
    println!("USAGE: {} <ip> <port>", bin_name);
}

fn client(mut stream: TcpStream) {
    let _ = stream.set_nonblocking(true);
    let mut pending: Vec<u8> = Vec::new();

    match read_line(&mut stream, &mut pending) {
        Ok(greeting) => {
            if !greeting.starts_with("200") {
                println!("Unexpected server greeting: {}", greeting);
            }
        }
        Err(e) => {
            println!("Failed to read server greeting: {}", e);
            return;
        }
    }

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line_result) = lines.next() {
        match line_result {
            Ok(line) => {
                let line_trimmed = line.trim();
                if !line_trimmed.starts_with('/') {
                    println!("Incorrect command!");
                    continue;
                }
                let without_slash = &line_trimmed[1..];
                let (cmd_name, cmd_args) = match without_slash.find(' ') {
                    Some(i) => {
                        let name = without_slash[..i].trim();
                        let rest = without_slash[i + 1..].trim_start();
                        (name, rest)
                    }
                    None => (without_slash, ""),
                };

                if let Some(handler) = commands().get(cmd_name) {
                    handler(&mut stream, &mut pending, cmd_args);
                } else {
                    println!("Unknown command!");
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

    let stream = TcpStream::connect(address)?;
    client(stream);
    Ok(())
}
