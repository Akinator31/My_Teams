use crate::clients::client::EventType::UserLoggedIn as EventUserLoggedIn;
use crate::clients::client::SuccessCode;
use crate::clients::client::SuccessCode::UserLoggedIn as CodeUserLoggedIn;
use crate::server::commands::check_command_format;
use crate::server::server::MyTeamsServer;
use crate::MAX_NAME_LENGTH;

pub fn login(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    if let Some(user_name) = check_command_format(&command_args) {
        let Some(user_index) = server.data.user_exist_by_name(&user_name) else {
            if user_name.len() > MAX_NAME_LENGTH {
                return false;
            }

            let new_user = server.data.create_user(&user_name);

            server.client_manager.clients[client_index].uuid = Some(new_user.uuid.clone());
            libs::ServerLog::server_event_user_created(
                new_user.uuid.clone(),
                new_user.user_name.clone(),
            );
            libs::ServerLog::server_event_user_logged_in(new_user.uuid.clone());

            server.client_manager.clients[client_index].write(SuccessCode::UserLoggedIn(new_user.uuid.clone()));

            server.send_global_event(EventUserLoggedIn(
                (
                    new_user.uuid.clone(),
                    new_user.user_name.clone()
                ).into()
            ), None);

            return true;
        };

        server.client_manager.clients[client_index].uuid = Some(server.data.users[user_index].uuid.clone());

        libs::ServerLog::server_event_user_logged_in(server.data.users[user_index].uuid.clone());

        server.client_manager.clients[client_index].write(CodeUserLoggedIn(server.data.users[user_index].uuid.clone()));

        server.send_global_event(EventUserLoggedIn(
            (
                server.data.users[user_index].uuid.clone(),
                server.data.users[user_index].user_name.clone()
            ).into()
        ), None);

        true
    } else {
        false
    }
}
