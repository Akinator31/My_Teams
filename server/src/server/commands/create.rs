use crate::clients::client::ErrorCode::{NotFound, Unauthorized};
use crate::clients::client::SuccessCode::Created;
use crate::clients::client::{CreatedResponse, EventType};
use crate::clients::context::{Context, ContextChannel, ContextTeam, ContextThread};
use crate::server::server::MyTeamsServer;
use crate::utils::parsing::parse_quoted_args;
use crate::{MAX_BODY_LENGTH, MAX_DESCRIPTION_LENGTH, MAX_NAME_LENGTH};

fn create_team(
    server: &mut MyTeamsServer,
    client_index: usize,
    team_name: String,
    team_description: String,
) -> bool {
    if team_name.len() > MAX_NAME_LENGTH || team_description.len() > MAX_DESCRIPTION_LENGTH {
        return false;
    }

    let client_uuid = server.client_manager.clients[client_index]
        .uuid
        .clone()
        .unwrap();

    let team = server.data.create_team(
        client_uuid.clone(),
        team_name.clone(),
        team_description.clone(),
    );

    server.client_manager.clients[client_index].write(Created(CreatedResponse::Team(team.0.clone())));
    server.send_global_event(
        EventType::TeamCreated(
            (
                team.0.clone(),
                team_name.clone(),
                team_description,
                client_uuid.clone(),
            )
                .into(),
        ),
        None,
    );

    libs::ServerLog::server_event_team_created(team.0, team_name, client_uuid);

    true
}

fn create_channel(
    server: &mut MyTeamsServer,
    client_index: usize,
    channel_name: String,
    channel_description: String,
    context: ContextTeam,
) -> bool {
    if channel_name.len() > MAX_NAME_LENGTH || channel_description.len() > MAX_DESCRIPTION_LENGTH {
        return false;
    }

    let Some(channel) = server.data.create_channel(
        context.team.clone(),
        channel_name.clone(),
        channel_description.clone(),
    ) else {
        server.client_manager.clients[client_index].write(NotFound);
        return true;
    };

    server.client_manager.clients[client_index].write(Created(CreatedResponse::Channel(channel.0.clone())));
    server.send_global_event(
        EventType::ChannelCreated(
            (
                channel.0.clone(),
                channel_name.clone(),
                channel_description,
                context.team.clone(),
            )
                .into(),
        ),
        Some(context.team.clone()),
    );

    libs::ServerLog::server_event_channel_created(context.team, channel.0, channel_name);

    true
}

fn create_thread(
    server: &mut MyTeamsServer,
    client_index: usize,
    thread_title: String,
    thread_body: String,
    context: ContextChannel,
) -> bool {
    if thread_title.len() > MAX_NAME_LENGTH || thread_body.len() > MAX_BODY_LENGTH {
        return false;
    }

    let client_uuid = server.client_manager.clients[client_index]
        .uuid
        .clone()
        .unwrap();

    let Some(thread) = server.data.create_thread(
        context.team.clone(),
        client_uuid.clone(),
        context.channel.clone(),
        thread_title.clone(),
        thread_body.clone(),
    ) else {
        server.client_manager.clients[client_index].write(NotFound);
        return true;
    };

    server.client_manager.clients[client_index].write(Created(CreatedResponse::Thread(thread.0.clone(), thread.2)));
    server.send_global_event(
        EventType::ThreadCreated(
            (
                thread.0.clone(),
                thread_title.clone(),
                thread_body.clone(),
                client_uuid.clone(),
                thread.2,
            )
                .into(),
        ),
        Some(context.team),
    );

    libs::ServerLog::server_event_thread_created(
        context.channel,
        thread.0,
        client_uuid,
        thread_title,
        thread_body,
    );

    true
}

fn create_reply(
    server: &mut MyTeamsServer,
    client_index: usize,
    reply_body: String,
    ctx: ContextThread,
) -> bool {
    if reply_body.len() > MAX_BODY_LENGTH {
        return false;
    }

    let client_uuid = server.client_manager.clients[client_index]
        .uuid
        .clone()
        .unwrap();

    let Some(reply) = server.data.create_reply(
        ctx.team.clone(),
        client_uuid.clone(),
        ctx.channel,
        ctx.thread.clone(),
        reply_body.clone(),
    ) else {
        server.client_manager.clients[client_index].write(NotFound);
        return true;
    };

    server.client_manager.clients[client_index].write(Created(CreatedResponse::Reply(ctx.thread.clone(), reply.2)));

    server.send_global_event(
        EventType::ReplyCreated(
            (
                ctx.team.clone(),
                reply.0.clone(),
                reply_body.clone(),
                client_uuid.clone(),
                ctx.thread.clone(),
            )
                .into(),
        ),
        Some(ctx.team),
    );

    libs::ServerLog::server_event_reply_created(ctx.thread, client_uuid, reply_body);

    true
}

pub fn create(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    if server.client_manager.clients[client_index].uuid.is_none() {
        server.client_manager.clients[client_index].write(Unauthorized);
        return true;
    }

    let args = parse_quoted_args(&command_args);
    let context = server.client_manager.clients[client_index].context.clone();

    match (context, args.as_slice()) {
        (Context::None, [team_name, team_description]) => create_team(
            server,
            client_index,
            team_name.clone(),
            team_description.clone(),
        ),
        (Context::Team(ctx), [channel_name, channel_description]) => create_channel(
            server,
            client_index,
            channel_name.clone(),
            channel_description.clone(),
            ctx,
        ),
        (Context::Channel(ctx), [thread_name, thread_description]) => create_thread(
            server,
            client_index,
            thread_name.clone(),
            thread_description.clone(),
            ctx,
        ),
        (Context::Thread(ctx), [reply_body]) => {
            create_reply(server, client_index, reply_body.clone(), ctx)
        }
        _ => false,
    }
}
