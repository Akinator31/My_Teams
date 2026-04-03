use crate::clients::client::SuccessCode::UserLoggedIn;
use crate::server::commands::check_command_format;
use crate::server::server::MyTeamsServer;
use crate::MAX_NAME_LENGTH;

pub fn login(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    let client = &mut server.client_manager.clients[client_index];

    if let Some(user_name) = check_command_format(&command_args) {
        if server.data.user_exist_by_name(&user_name).is_none() {
            if user_name.len() > MAX_NAME_LENGTH as usize {
                return false;
            }

            let new_user = server.data.create_user(&user_name);

            client.uuid = Some(new_user.uuid.clone());
            libs::ServerLog::server_event_user_created(
                new_user.uuid.clone(),
                new_user.user_name.clone(),
            );
            libs::ServerLog::server_event_user_logged_in(new_user.uuid.clone());

            client.write(UserLoggedIn(new_user.uuid.clone()));
        } else {
            if let Some(user) = server.data.get_user(&user_name) {
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
