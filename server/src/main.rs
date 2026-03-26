mod errors;
mod clients;
mod server;
mod utils;

use std::error::Error;
use std::sync::atomic::Ordering;
use crate::server::MyTeamsServer;
use crate::utils::signals::{setup_signal_handler, SHUTDOWN};

fn main() -> Result<(), Box<dyn Error>> {
    let mut server = MyTeamsServer::new()?;

    server.client_manager.set_nonblocking(true)?;
    setup_signal_handler();

    loop {
        if (SHUTDOWN.load(Ordering::SeqCst)) {
            println!("Shutting down MyTeams server...");
            break;
        }

        server.handle_incoming_connections()?;
    }

    Ok(())
}
