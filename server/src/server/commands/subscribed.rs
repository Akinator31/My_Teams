use crate::clients::client::ErrorCode::{NotFound, Unauthorized};
use crate::clients::client::OkeyResponse::{EnfOfSubscribedTeams, EnfOfSubscribedUsers};
use crate::clients::client::SuccessCode::{
    Okay, SubscribedTeamsListFollows, SubscribedUsersListFollows,
};
use crate::server::server::MyTeamsServer;
use crate::utils::parsing::parse_quoted_args;

fn list_team_subscribed_user(server: &mut MyTeamsServer, client_index: usize, team_uuid: String) {
    let Some(team_index) = server.data.find_teams(team_uuid) else {
        server.client_manager.clients[client_index].write(NotFound);
        return;
    };

    for user_uuid in &server.data.teams[team_index].subscribed {
        server.client_manager.clients[client_index]
            .write(SubscribedUsersListFollows(user_uuid.clone()))
    }

    server.client_manager.clients[client_index].write(Okay(EnfOfSubscribedUsers));
}

fn list_user_subscribed_teams(
    server: &mut MyTeamsServer,
    client_uuid: &String,
    client_index: usize,
) {
    for team in &server.data.teams {
        if team.subscribed.contains(&client_uuid) {
            server.client_manager.clients[client_index]
                .write(SubscribedTeamsListFollows(team.uuid.clone()))
        }
    }

    server.client_manager.clients[client_index].write(Okay(EnfOfSubscribedTeams));
}

pub fn subscribed(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    let client_uuid = match server.client_manager.clients[client_index].uuid.clone() {
        Some(uuid) => uuid,
        None => {
            server.client_manager.clients[client_index].write(Unauthorized);
            return true;
        }
    };

    let args = parse_quoted_args(&command_args);
    match args.as_slice() {
        [team_uuid] => list_team_subscribed_user(server, client_index, team_uuid.clone()),
        [] => list_user_subscribed_teams(server, &client_uuid, client_index),
        _ => {
            return false;
        }
    }

    true
}
