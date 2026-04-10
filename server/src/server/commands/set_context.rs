use crate::clients::client::ErrorCode::Unauthorized;
use crate::clients::client::SuccessCode::ContextSet;
use crate::clients::context::{Context, ContextChannel, ContextTeam, ContextThread};
use crate::server::server::MyTeamsServer;
use crate::utils::parsing::parse_quoted_args;

fn set_context_to_none(server: &mut MyTeamsServer, client_index: usize) -> bool {
    server.client_manager.clients[client_index].context = Some(Context::None);
    server.client_manager.clients[client_index].write(ContextSet(Context::None));

    true
}

fn set_context_to_team(server: &mut MyTeamsServer, client_index: usize, team_uuid: String) -> bool {
    let context = Context::Team(ContextTeam { team: team_uuid });

    server.client_manager.clients[client_index].context = Some(context.clone());
    server.client_manager.clients[client_index].write(ContextSet(context));

    true
}

fn set_context_to_channel(
    server: &mut MyTeamsServer,
    client_index: usize,
    team_uuid: String,
    channel_uuid: String,
) -> bool {
    let context = Context::Channel(ContextChannel {
        team: team_uuid,
        channel: channel_uuid,
    });

    server.client_manager.clients[client_index].context = Some(context.clone());
    server.client_manager.clients[client_index].write(ContextSet(context));
    true
}

fn set_context_to_thread(
    server: &mut MyTeamsServer,
    client_index: usize,
    team_uuid: String,
    channel_uuid: String,
    thread_uuid: String,
) -> bool {
    let context = Context::Thread(ContextThread {
        team: team_uuid,
        channel: channel_uuid,
        thread: thread_uuid,
    });

    server.client_manager.clients[client_index].context = Some(context.clone());
    server.client_manager.clients[client_index].write(ContextSet(context));
    true
}

pub fn set_context(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    let _ = match server.client_manager.clients[client_index].uuid.clone() {
        Some(uuid) => uuid,
        None => {
            server.client_manager.clients[client_index].write(Unauthorized);
            return true;
        }
    };

    let args = parse_quoted_args(&command_args);
    match args.as_slice() {
        [] => set_context_to_none(server, client_index),
        [team] => set_context_to_team(server, client_index, team.clone()),
        [team, channel] => {
            set_context_to_channel(server, client_index, team.clone(), channel.clone())
        }
        [team, channel, thread] => set_context_to_thread(
            server,
            client_index,
            team.clone(),
            channel.clone(),
            thread.clone(),
        ),
        _ => false,
    }
}
