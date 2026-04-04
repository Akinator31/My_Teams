use crate::clients::client::ErrorCode::Unauthorized;
use crate::clients::client::OkeyResponse::EndOfUsers;
use crate::clients::client::SuccessCode::{Okay, UsersListFollows};
use crate::server::commands::check_command_format;
use crate::server::server::MyTeamsServer;

pub fn users(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    let _ = match server.client_manager.clients[client_index].uuid.clone() {
        Some(uuid) => uuid,
        None => {
            server.client_manager.clients[client_index].write(Unauthorized);
            return true;
        }
    };

    let Some(_): Option<()> = check_command_format(&command_args) else {
        return false;
    };

    for user in &server.data.users {
        let is_logged_in = match server.client_manager.is_client_logged_in(user.uuid.clone()) {
            Some(_) => true,
            None => false,
        };

        server.client_manager.clients[client_index]
            .write(UsersListFollows(user.clone(), is_logged_in));
    }

    server.client_manager.clients[client_index].write(Okay(EndOfUsers));

    true
}
