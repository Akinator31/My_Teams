use crate::clients::client::ErrorCode::{NotFound, Unauthorized};
use crate::clients::client::EventType::MessageReceived;
use crate::server::data::UserPair;
use crate::server::server::MyTeamsServer;
use crate::utils::parsing::parse_quoted_args;

fn check_command_format(command_args: &str) -> Option<(String, String)> {
    let args = parse_quoted_args(command_args);

    if args.len() == 2 {
        Some((args[0].clone(), args[1].clone()))
    } else {
        None
    }
}

pub fn send(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    if server.client_manager.clients[client_index].uuid.is_none() {
        server.client_manager.clients[client_index].write(Unauthorized);
        return true;
    }

    let client_uuid = match server.client_manager.clients[client_index].uuid.clone() {
        Some(uuid) => uuid,
        None => {
            println!("User not connected!");
            return false;
        }
    };

    let Some(args) = check_command_format(&command_args) else {
        return false;
    };

    let receiver_uuid = &args.0.to_string();
    let message = &args.1.to_string();

    let receiver_index = match server.data.user_exist(&receiver_uuid) {
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
        server.client_manager.clients[receiver_index]
            .send_event(MessageReceived(client_uuid, message.clone()));
    }

    server
        .data
        .register_send_message(users_pair, message.clone());

    true
}
