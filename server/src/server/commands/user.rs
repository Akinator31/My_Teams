use crate::clients::client::ErrorCode::Unauthorized;
use crate::clients::client::{ErrorCode, SuccessCode};
use crate::server::commands::check_command_format;
use crate::server::server::MyTeamsServer;

pub fn user(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    let _ = match server.client_manager.clients[client_index].uuid.clone() {
        Some(uuid) => uuid,
        None => {
            server.client_manager.clients[client_index].write(Unauthorized);
            return true;
        }
    };

    let Some(user_uuid): Option<String> = check_command_format(&command_args) else {
        return false;
    };

    let Some(user) = server
        .data
        .users
        .iter()
        .find(|&user| user.uuid == user_uuid)
    else {
        server.client_manager.clients[client_index].write(ErrorCode::UserNotFound(user_uuid));
        return true;
    };

    let is_logged_in = match server.client_manager.is_client_logged_in(user.uuid.clone()) {
        Some(_) => true,
        None => false,
    };

    server.client_manager.clients[client_index]
        .write(SuccessCode::UserInfoFollows(user.clone(), is_logged_in));

    true
}
