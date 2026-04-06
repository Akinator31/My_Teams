use crate::clients::client::OkeyResponse::EndOfHelp;
use crate::clients::client::SuccessCode::{AvailableCommands, Okay};
use crate::server::server::MyTeamsServer;

pub fn help(server: &mut MyTeamsServer, client_index: usize, _command_args: String) -> bool {
    server.client_manager.clients[client_index].write(AvailableCommands);

    server.client_manager.clients[client_index].write(Okay(EndOfHelp));

    true
}
