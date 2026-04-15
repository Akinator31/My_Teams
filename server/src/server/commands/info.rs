use crate::clients::client::ErrorCode::{ChannelNotFound, TeamNotFound, ThreadNotFound, Unauthorized};
use crate::clients::client::SuccessCode::{
    InfoChannelFollows, InfoTeamFollows, InfoThreadFollows, InfoUserFollows,
};
use crate::clients::context::{Context, ContextChannel, ContextTeam, ContextThread};
use crate::server::server::MyTeamsServer;
use crate::utils::parsing::parse_quoted_args;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UserInfo {
    pub uuid: String,
    pub username: String,
    pub is_logged: bool,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TeamInfo {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub creator_uuid: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ChannelInfo {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub team_uuid: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ThreadInfo {
    pub uuid: String,
    pub title: String,
    pub message: String,
    pub creator_uuid: String,
    pub timestamp: i64,
}

fn user_info(server: &mut MyTeamsServer, client_index: usize) {
    let client = server
        .data
        .get_user_by_uuid(
            &server.client_manager.clients[client_index]
                .uuid
                .clone()
                .unwrap(),
        )
        .unwrap();

    let is_logged = server
        .client_manager
        .is_client_logged_in(client.uuid.clone())
        .is_some();

    server.client_manager.clients[client_index].write(InfoUserFollows(
        (client.uuid.clone(), client.user_name.clone(), is_logged).into(),
    ))
}

fn team_info(server: &mut MyTeamsServer, client_index: usize, ctx: ContextTeam) {
    let Some(team_index) = server.data.find_teams_by_uuid(ctx.team.clone()) else {
        server.client_manager.clients[client_index].write(TeamNotFound(ctx.team));
        return;
    };

    let team = &server.data.teams[team_index];

    server.client_manager.clients[client_index].write(InfoTeamFollows(
        (
            team.uuid.clone(),
            team.name.clone(),
            team.description.clone(),
            team.author.clone(),
        )
            .into(),
    ))
}

fn channel_info(server: &mut MyTeamsServer, client_index: usize, ctx: ContextChannel) {
    let Some(team_index) = server.data.find_teams_by_uuid(ctx.team.clone()) else {
        server.client_manager.clients[client_index].write(TeamNotFound(ctx.team));
        return;
    };

    let Some(channel_index) = server.data.find_channels_by_uuid(team_index, ctx.channel.clone()) else {
        server.client_manager.clients[client_index].write(ChannelNotFound(ctx.channel));
        return;
    };

    let channel = &server.data.teams[team_index].channels[channel_index];

    server.client_manager.clients[client_index].write(InfoChannelFollows(
        (
            channel.uuid.clone(),
            channel.name.clone(),
            channel.description.clone(),
            channel.team.clone(),
        )
            .into(),
    ))
}

fn thread_info(server: &mut MyTeamsServer, client_index: usize, ctx: ContextThread) {
    let Some(team_index) = server.data.find_teams_by_uuid(ctx.team.clone()) else {
        server.client_manager.clients[client_index].write(TeamNotFound(ctx.team));
        return;
    };

    let Some(channel_index) = server.data.find_channels_by_uuid(team_index, ctx.channel.clone()) else {
        server.client_manager.clients[client_index].write(ChannelNotFound(ctx.channel));
        return;
    };

    let Some(thread_index) = server
        .data
        .find_threads_by_uuid(team_index, channel_index, ctx.thread.clone())
    else {
        server.client_manager.clients[client_index].write(ThreadNotFound(ctx.thread));
        return;
    };

    let thread = &server.data.teams[team_index].channels[channel_index].threads[thread_index];

    server.client_manager.clients[client_index].write(InfoThreadFollows(
        (
            thread.uuid.clone(),
            thread.title.clone(),
            thread.body.clone(),
            thread.author.clone(),
            thread.timestamp,
        )
            .into(),
    ))
}

pub fn info(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    if server.client_manager.clients[client_index].uuid.is_none() {
        server.client_manager.clients[client_index].write(Unauthorized);
        return true;
    }

    let args = parse_quoted_args(&command_args);
    let context = server.client_manager.clients[client_index].context.clone();

    match (context, args.as_slice()) {
        (Context::None, []) => user_info(server, client_index),
        (Context::Team(ctx), []) => team_info(server, client_index, ctx),
        (Context::Channel(ctx), []) => channel_info(server, client_index, ctx),
        (Context::Thread(ctx), []) => thread_info(server, client_index, ctx),
        _ => return false,
    }

    true
}

impl From<(String, String, bool)> for UserInfo {
    fn from(value: (String, String, bool)) -> Self {
        UserInfo {
            uuid: value.0,
            username: value.1,
            is_logged: value.2,
        }
    }
}

impl From<(String, String, String, String)> for TeamInfo {
    fn from(value: (String, String, String, String)) -> Self {
        TeamInfo {
            uuid: value.0,
            name: value.1,
            description: value.2,
            creator_uuid: value.3,
        }
    }
}

impl From<(String, String, String, String)> for ChannelInfo {
    fn from(value: (String, String, String, String)) -> Self {
        ChannelInfo {
            uuid: value.0,
            name: value.1,
            description: value.2,
            team_uuid: value.3,
        }
    }
}

impl From<(String, String, String, String, i64)> for ThreadInfo {
    fn from(value: (String, String, String, String, i64)) -> Self {
        ThreadInfo {
            uuid: value.0,
            title: value.1,
            message: value.2,
            creator_uuid: value.3,
            timestamp: value.4,
        }
    }
}
