mod clients;
mod errors;
mod server;
mod utils;

use crate::server::server::MyTeamsServer;
use crate::utils::signals::{setup_signal_handler, SHUTDOWN};
use std::error::Error;
use std::sync::atomic::Ordering;

static MAX_NAME_LENGTH: usize = 32;
static MAX_DESCRIPTION_LENGTH: usize = 255;
static MAX_BODY_LENGTH: usize = 512;

fn main() -> Result<(), Box<dyn Error>> {
    let mut server = MyTeamsServer::new()?;

    server.client_manager.set_nonblocking(true)?;
    setup_signal_handler();

    loop {
        if (SHUTDOWN.load(Ordering::SeqCst)) {
            println!("Shutting down MyTeams server...");
            break;
        }

        server.client_manager.connect_client()?;

        server.client_manager.receive_clients_data()?;

        server.client_manager.disconnect_client()?;

        server.execute_clients_pending_command();
    }

    Ok(())
}
