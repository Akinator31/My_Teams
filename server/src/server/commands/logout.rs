use crate::clients::client::SuccessCode::UserLoggedOut;
use crate::server::server::MyTeamsServer;

fn check_command_format(command_args: &str) -> Option<String> {
    let mut parts = command_args.split(' ');
    match (parts.next(), parts.next()) {
        (Some(name), None) => Some(name.trim().to_string()),
        _ => None,
    }
}

pub fn logout(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    let client = &mut server.client_manager.clients[client_index];

    if let Some(_) = check_command_format(&command_args) {
        if client.uuid.is_none() {
            return false;
        }

        libs::ServerLog::server_event_user_logged_out(client.uuid.clone().unwrap().to_string());
        client.uuid = None;
        client.write(UserLoggedOut);
        true
    } else {
        false
    }
}
