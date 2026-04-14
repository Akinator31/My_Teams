use crate::clients::client::ErrorCode::{NotFound, Unauthorized};
use crate::clients::client::OkeyResponse::{
    EndOfChannelsList, EndOfRepliesList, EndOfTeamsList, EndOfThreadsList,
};
use crate::clients::client::SuccessCode::{
    ChannelsListFollows, Okay, RepliesListFollows, TeamsListFollows, ThreadsListFollows,
};
use crate::clients::context::{Context, ContextChannel, ContextTeam, ContextThread};
use crate::server::server::MyTeamsServer;
use crate::utils::parsing::parse_quoted_args;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ListTeams {
    pub team_uuid: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ListChannels {
    pub channel_uuid: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ListThreads {
    pub thread_uuid: String,
    pub title: String,
    pub creator_uuid: String,
    pub timestamp: i64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ListReplies {
    pub comment_uuid: String,
    pub body: String,
    pub creator_uuid: String,
    pub timestamp: i64,
}

fn list_teams(server: &mut MyTeamsServer, client_index: usize) {
    for team in &server.data.teams {
        server.client_manager.clients[client_index].write(TeamsListFollows(
            (
                team.uuid.clone(),
                team.name.clone(),
                team.description.clone(),
            )
                .into(),
        ));
    }

    server.client_manager.clients[client_index].write(Okay(EndOfTeamsList));
}

fn list_channels(server: &mut MyTeamsServer, client_index: usize, ctx: ContextTeam) {
    let Some(team_index) = server.data.find_teams(ctx.team) else {
        server.client_manager.clients[client_index].write(NotFound);
        return;
    };

    for channel in &server.data.teams[team_index].channels {
        server.client_manager.clients[client_index].write(ChannelsListFollows(
            (
                channel.uuid.clone(),
                channel.name.clone(),
                channel.description.clone(),
            )
                .into(),
        ));
    }

    server.client_manager.clients[client_index].write(Okay(EndOfChannelsList));
}

fn list_threads(server: &mut MyTeamsServer, client_index: usize, ctx: ContextChannel) {
    let Some(team_index) = server.data.find_teams(ctx.team.clone()) else {
        server.client_manager.clients[client_index].write(NotFound);
        return;
    };

    let Some(channel_index) = server.data.find_channels(team_index, ctx.channel) else {
        server.client_manager.clients[client_index].write(NotFound);
        return;
    };

    for thread in &server.data.teams[team_index].channels[channel_index].threads {
        server.client_manager.clients[client_index].write(ThreadsListFollows(
            (
                thread.uuid.clone(),
                thread.title.clone(),
                thread.author.clone(),
                thread.timestamp,
            )
                .into(),
        ))
    }

    server.client_manager.clients[client_index].write(Okay(EndOfThreadsList));
}

fn list_replies(server: &mut MyTeamsServer, client_index: usize, ctx: ContextThread) {
    let Some(team_index) = server.data.find_teams(ctx.team.clone()) else {
        server.client_manager.clients[client_index].write(NotFound);
        return;
    };

    let Some(channel_index) = server.data.find_channels(team_index, ctx.channel) else {
        server.client_manager.clients[client_index].write(NotFound);
        return;
    };

    let Some(thread_index) = server
        .data
        .find_threads(team_index, channel_index, ctx.thread)
    else {
        server.client_manager.clients[client_index].write(NotFound);
        return;
    };

    for reply in
        &server.data.teams[team_index].channels[channel_index].threads[thread_index].comments
    {
        server.client_manager.clients[client_index].write(RepliesListFollows(
            (
                reply.uuid.clone(),
                reply.body.clone(),
                reply.user_uuid.clone(),
                reply.timestamp,
            )
                .into(),
        ))
    }

    server.client_manager.clients[client_index].write(Okay(EndOfRepliesList));
}

pub fn list(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    if server.client_manager.clients[client_index].uuid.is_none() {
        server.client_manager.clients[client_index].write(Unauthorized);
        return true;
    }

    let args = parse_quoted_args(&command_args);
    let context = server.client_manager.clients[client_index].context.clone();

    match (context, args.as_slice()) {
        (Context::None, []) => list_teams(server, client_index),
        (Context::Team(ctx), []) => list_channels(server, client_index, ctx),
        (Context::Channel(ctx), []) => list_threads(server, client_index, ctx),
        (Context::Thread(ctx), []) => list_replies(server, client_index, ctx),
        _ => return false,
    }

    true
}

impl From<(String, String, String)> for ListTeams {
    fn from(value: (String, String, String)) -> Self {
        ListTeams {
            team_uuid: value.0,
            name: value.1,
            description: value.2,
        }
    }
}

impl From<(String, String, String)> for ListChannels {
    fn from(value: (String, String, String)) -> Self {
        ListChannels {
            channel_uuid: value.0,
            name: value.1,
            description: value.2,
        }
    }
}

impl From<(String, String, String, i64)> for ListThreads {
    fn from(value: (String, String, String, i64)) -> Self {
        ListThreads {
            thread_uuid: value.0,
            title: value.1,
            creator_uuid: value.2,
            timestamp: value.3,
        }
    }
}

impl From<(String, String, String, i64)> for ListReplies {
    fn from(value: (String, String, String, i64)) -> Self {
        ListReplies {
            comment_uuid: value.0,
            body: value.1,
            creator_uuid: value.2,
            timestamp: value.3,
        }
    }
}
