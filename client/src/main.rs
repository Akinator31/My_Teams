mod commands;
mod errors;

use std::env::args;
use std::io::BufRead;
use std::process::exit;
use crate::commands::commands::commands;

fn client() {
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

                if let Some(command) = commands().get(command) {
                    command();
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

fn main() {
    let args = args();

    if args.len() != 3 {
        println!("NON");
        exit(84);
    }
}
