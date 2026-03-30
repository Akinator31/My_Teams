use crate::clients::client::Client;
use crate::clients::client::ReplyCode::UserLoggedIn;
use crate::MAX_NAME_LENGTH;
use crate::server::data::MyTeamsServerData;

fn check_command_format(command_args: &str) -> Option<String> {
    let mut parts = command_args.split(' ');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(_cmd), Some(name), None) => Some(name.trim().to_string()),
        _ => None,
    }
}

pub fn login(server_data: &mut MyTeamsServerData, client: &mut Client, command_args: String) -> bool {
    if let Some(user_name) = check_command_format(&command_args) {

        if !server_data.user_exist(&user_name) {

            if user_name.len() > MAX_NAME_LENGTH as usize {
                return false;
            }

            let new_user = server_data.create_user(&user_name);

            client.uuid = Some(new_user.uuid.clone());
            libs::ServerLog::server_event_user_created(new_user.uuid.clone(), new_user.user_name.clone());
            libs::ServerLog::server_event_user_logged_in(new_user.uuid.clone());

            client.write(UserLoggedIn(new_user.uuid.clone()));
        } else {
            if let Some(user) = server_data.get_user(&user_name) {
                client.uuid = Some(user.uuid.clone());

                libs::ServerLog::server_event_user_logged_in(user.uuid.clone());

                client.write(UserLoggedIn(user.uuid.clone()));
            }
        }

        true
    } else {
        false
    }
}