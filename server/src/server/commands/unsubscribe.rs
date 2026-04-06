use crate::clients::client::ErrorCode::{NotFound, Unauthorized};
use crate::clients::client::EventType;
use crate::clients::client::OkeyResponse::UnsubscribedToTeam;
use crate::clients::client::SuccessCode::Okay;
use crate::server::commands::check_command_format;
use crate::server::server::MyTeamsServer;

pub fn unsubscribe(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    let Some(user_uuid) = server.client_manager.clients[client_index].uuid.clone() else {
        server.client_manager.clients[client_index].write(Unauthorized);
        return true;
    };

    let Some(team_uuid): Option<String> = check_command_format(&command_args) else {
        return false;
    };

    if !server
        .data
        .unsubscribe_from_team(team_uuid.clone(), user_uuid.clone())
    {
        server.client_manager.clients[client_index].write(NotFound);
    } else {
        server.client_manager.clients[client_index].write(Okay(UnsubscribedToTeam));

        server.send_global_event(
            EventType::UserUnsubscribed((user_uuid.clone(), team_uuid.clone()).into()),
            Some(team_uuid.clone()),
        )
    }

    libs::ServerLog::server_event_user_unsubscribed(team_uuid, user_uuid);
    true
}
