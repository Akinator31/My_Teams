use crate::clients::client::ErrorCode::{Unauthorized, UserNotFound};
use crate::clients::client::SuccessCode::UserLoggedOut;
use crate::server::commands::check_command_format;
use crate::server::server::MyTeamsServer;

pub fn logout(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    let client_uuid = match server.client_manager.clients[client_index].uuid.clone() {
        Some(uuid) => uuid,
        None => {
            server.client_manager.clients[client_index].write(Unauthorized);
            return true;
        }
    };

    if let Some(_) = check_command_format::<()>(&command_args) {
        if server.client_manager.clients[client_index].uuid.is_none() {
            return false;
        }

        let Some(user_index) = server.data.user_exist_by_uuid(&client_uuid) else {
            server.client_manager.clients[client_index].write(UserNotFound(client_uuid));
            return true;
        };

        libs::ServerLog::server_event_user_logged_out(
            server.client_manager.clients[client_index]
                .uuid
                .clone()
                .unwrap()
                .to_string(),
        );
        server.client_manager.clients[client_index].write(UserLoggedOut);

        server.send_global_event(
            crate::clients::client::EventType::UserLoggedOut(
                (
                    server.data.users[user_index].uuid.clone(),
                    server.data.users[user_index].user_name.clone(),
                )
                    .into(),
            ),
            None,
        );

        server.client_manager.clients[client_index].uuid = None;

        true
    } else {
        false
    }
}
