mod client;
mod commands;
mod errors;
mod events;
mod transport;
mod utils;

use crate::client::Client;
use crate::utils::signals::{SHUTDOWN, setup_signal_handler};
use std::error::Error;
use std::sync::atomic::Ordering;

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::args().any(|arg| arg == "--help") {
        utils::help::show_help();
        return Ok(());
    }

    let mut client = Client::new()?;

    client.connect()?;
    setup_signal_handler();

    loop {
        if SHUTDOWN.load(Ordering::SeqCst) {
            println!("Shutting down MyTeams client...");
            break;
        }

        client.poll()?;

        while let Some(message) = client.get_pending_server_message() {
            client.execute_server_event(message);
        }

        while let Some(line) = client.get_pending_stdin_line() {
            client.execute_stdin_command(line);
        }

        if client.is_server_disconnected() {
            println!("Server disconnected.");
            break;
        }
    }

    Ok(())
}
