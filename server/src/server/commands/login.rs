use crate::clients::client::Client;
use crate::clients::client::ReplyCode::UserLoggedIn;
use crate::server::data::MyTeamsServerData;

fn check_command_format(command_args: String) -> Option<String> {
    if command_args.split(" ").count() != 2 {
        return None;
    }
    Some(command_args.split("").next().unwrap_or("").to_string())
}

pub fn login(server_data: &mut MyTeamsServerData, client: &mut Client, command_args: String) -> bool {
    if let Some(user_name) = check_command_format(command_args) {

        if !server_data.user_exist(&user_name) {
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