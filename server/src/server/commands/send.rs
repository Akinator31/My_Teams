use crate::clients::client::ErrorCode::{NotFound, Unauthorized};
use crate::clients::client::EventType::MessageReceived;
use crate::clients::client::SuccessCode::MessageSent;
use crate::server::commands::check_command_format;
use crate::server::data::UserPair;
use crate::server::server::MyTeamsServer;

pub fn send(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    let client_uuid = match server.client_manager.clients[client_index].uuid.clone() {
        Some(uuid) => uuid,
        None => {
            server.client_manager.clients[client_index].write(Unauthorized);
            return true;
        }
    };

    let Some(args): Option<(String, String)> = check_command_format(&command_args) else {
        return false;
    };

    let receiver_uuid = &args.0.to_string();
    let message = &args.1.to_string();

    let receiver_index = match server.data.user_exist_by_uuid(&receiver_uuid) {
        Some(index) => index,
        None => {
            server.client_manager.clients[client_index].write(NotFound);
            return true;
        }
    };

    let users_pair = UserPair(client_uuid.clone(), receiver_uuid.clone());

    if let Some(_) = server
        .client_manager
        .is_client_logged_in(receiver_uuid.clone())
    {
        server.client_manager.clients[receiver_index].send_event(MessageReceived(
            (client_uuid.clone(), message.clone()).into(),
        ));
    }

    server
        .data
        .register_send_message(users_pair, message.clone());

    server.client_manager.clients[client_index].write(MessageSent);

    libs::ServerLog::server_event_private_message_sended(
        client_uuid,
        receiver_uuid.clone(),
        message.clone(),
    );

    true
}
