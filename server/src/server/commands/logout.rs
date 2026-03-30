use crate::clients::client::Client;
use crate::clients::client::ReplyCode::UserLoggedOut;
use crate::server::data::MyTeamsServerData;

fn check_command_format(command_args: &str) -> Option<String> {
    let mut parts = command_args.split(' ');
    match (parts.next(), parts.next()) {
        (Some(name), None) => Some(name.trim().to_string()),
        _ => None,
    }
}

pub fn logout(_server_data: &mut MyTeamsServerData, client: &mut Client, command_args: String) -> bool {
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