use crate::clients::client::ErrorCode::{Unauthorized, UserNotFound};
use crate::clients::client::OkeyResponse::EndOfMessages;
use crate::clients::client::SuccessCode::{MessageListFollows, Okay};
use crate::server::commands::check_command_format;
use crate::server::data::UserPair;
use crate::server::server::MyTeamsServer;

pub fn messages(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    let client_uuid = match server.client_manager.clients[client_index].uuid.clone() {
        Some(uuid) => uuid,
        None => {
            server.client_manager.clients[client_index].write(Unauthorized);
            return true;
        }
    };

    let Some(user_uuid): Option<String> = check_command_format(&command_args) else {
        return false;
    };

    let Some(_) = server.data.user_exist_by_uuid(&user_uuid) else {
        server.client_manager.clients[client_index].write(UserNotFound(user_uuid));
        return true;
    };

    let Some(messages) = server.data.get_messages(UserPair(client_uuid, user_uuid)) else {
        server.client_manager.clients[client_index].write(Okay(EndOfMessages));
        return true;
    };

    for message in messages {
        server.client_manager.clients[client_index].write(MessageListFollows(message))
    }

    server.client_manager.clients[client_index].write(Okay(EndOfMessages));

    true
}
