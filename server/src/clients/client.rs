use crate::clients::client::ClientState::ToBeDisconnected;
use crate::clients::context::Context;
use crate::errors::myteams_errors::MyTeamsServerError;
use crate::server::commands::info::{ChannelInfo, TeamInfo, ThreadInfo, UserInfo};
use crate::server::commands::list::{ListChannels, ListReplies, ListTeams, ListThreads};
use crate::server::data::channel::ChannelCreatedEvent;
use crate::server::data::team::TeamCreatedEvent;
use crate::server::data::thread::{ReplyCreatedEvent, ThreadCreatedEvent};
use crate::server::data::user::{
    User, UserLoggedInEvent, UserLoggedOutEvent, UserSubscribedEvent, UserUnsubscribedEvent,
};
use crate::server::data::{Message, MessageCreatedEvent};
use std::fmt::{Display, Formatter};
use std::io::ErrorKind::WouldBlock;
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug, PartialEq)]
pub enum ClientState {
    None,
    ToBeDisconnected,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OkeyResponse {
    Connected,
    EndOfMessages,
    EndOfUsers,
    SubscribedToTeam,
    UnsubscribedToTeam,
    EnfOfSubscribedTeams,
    EnfOfSubscribedUsers,
    EndOfTeamsList,
    EndOfChannelsList,
    EndOfThreadsList,
    EndOfRepliesList,
    EndOfHelp,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SuccessCode {
    Okay(OkeyResponse),
    Created,
    UserLoggedIn(String),
    UserLoggedOut,
    InfoUserFollows(UserInfo),
    MessageSent,
    MessageListFollows(Message),
    TeamsListFollows(ListTeams),
    ChannelsListFollows(ListChannels),
    ThreadsListFollows(ListThreads),
    RepliesListFollows(ListReplies),
    UsersListFollows(User, bool),
    UserInfoFollows(User, bool),
    SubscribedTeamsListFollows(String),
    SubscribedUsersListFollows(String),
    ContextSet(Context),
    InfoTeamFollows(TeamInfo),
    InfoChannelFollows(ChannelInfo),
    InfoThreadFollows(ThreadInfo),
    AvailableCommands,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    BadRequest,
    NotFound,
    Unauthorized,
    NoContextSet,
}

#[derive(Clone, Debug)]
pub enum EventType {
    UserLoggedIn(UserLoggedInEvent),
    UserLoggedOut(UserLoggedOutEvent),
    TeamCreated(TeamCreatedEvent),
    ChannelCreated(ChannelCreatedEvent),
    ThreadCreated(ThreadCreatedEvent),
    ReplyCreated(ReplyCreatedEvent),
    MessageReceived(MessageCreatedEvent),
    UserSubscribed(UserSubscribedEvent),
    UserUnsubscribed(UserUnsubscribedEvent),
}

impl Display for OkeyResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OkeyResponse::Connected => write!(f, "Connected to MyTeams server"),
            OkeyResponse::EndOfMessages => write!(f, "End of messages"),
            OkeyResponse::EndOfUsers => write!(f, "End of users list"),
            OkeyResponse::SubscribedToTeam => write!(f, "Subscribed to a team"),
            OkeyResponse::UnsubscribedToTeam => write!(f, "Unsubscribed to a team"),
            OkeyResponse::EnfOfSubscribedTeams => write!(f, "End of subscribed teams"),
            OkeyResponse::EnfOfSubscribedUsers => write!(f, "End of subscribed users"),
            OkeyResponse::EndOfTeamsList => write!(f, "End of teams list"),
            OkeyResponse::EndOfChannelsList => write!(f, "End of channels list"),
            OkeyResponse::EndOfThreadsList => write!(f, "End of threads list"),
            OkeyResponse::EndOfRepliesList => write!(f, "End of replies list"),
            OkeyResponse::EndOfHelp => write!(f, "End of help"),
        }
    }
}

impl From<SuccessCode> for String {
    fn from(value: SuccessCode) -> Self {
        match value {
            SuccessCode::Okay(message) => format!("200 {}\r\n", message),
            SuccessCode::Created => "201 Created\r\n".to_string(),
            SuccessCode::UserLoggedIn(username) => {
                format!("210 User logged in. UUID: {}\r\n", username)
            }
            SuccessCode::UserLoggedOut => "211 User logged out\r\n".to_string(),
            SuccessCode::InfoUserFollows(user) => {
                format!(
                    "213 \"{}\" \"{}\" \"{}\"\r\n",
                    user.uuid, user.username, user.is_logged
                )
            }
            SuccessCode::MessageSent => "220 Message sent\r\n".to_string(),
            SuccessCode::MessageListFollows(message) => {
                format!(
                    "221 \"{}\" \"{}\" \"{}\"\r\n",
                    message.sender, message.timestamp, message.body
                )
            }
            SuccessCode::TeamsListFollows(list) => {
                format!(
                    "230 \"{}\" \"{}\" \"{}\"\r\n",
                    list.team_uuid, list.name, list.description
                )
            }
            SuccessCode::ChannelsListFollows(list) => {
                format!(
                    "233 \"{}\" \"{}\" \"{}\"\r\n",
                    list.channel_uuid, list.name, list.description
                )
            }
            SuccessCode::ThreadsListFollows(list) => {
                format!(
                    "234 \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
                    list.thread_uuid, list.title, list.creator_uuid, list.timestamp
                )
            }
            SuccessCode::RepliesListFollows(list) => {
                format!(
                    "235 \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
                    list.comment_uuid, list.body, list.creator_uuid, list.timestamp
                )
            }
            SuccessCode::UsersListFollows(user, is_logged_in) => {
                format!(
                    "212 \"{}\" \"{}\" \"{}\"\r\n",
                    user.uuid, user.user_name, is_logged_in
                )
            }
            SuccessCode::UserInfoFollows(user, is_logged_in) => {
                format!(
                    "213 \"{}\" \"{}\" \"{}\"\r\n",
                    user.uuid, user.user_name, is_logged_in
                )
            }
            SuccessCode::SubscribedTeamsListFollows(team_uuid) => {
                format!("231 \"{}\"\r\n", team_uuid)
            }
            SuccessCode::SubscribedUsersListFollows(user_uuid) => {
                format!("232 \"{}\"\r\n", user_uuid)
            }
            SuccessCode::ContextSet(context) => {
                format!("250 Context set to {}\r\n", context)
            }
            SuccessCode::InfoTeamFollows(team) => {
                format!(
                    "240 \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
                    team.uuid, team.name, team.description, team.creator_uuid
                )
            }
            SuccessCode::InfoChannelFollows(channel) => {
                format!(
                    "241 \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
                    channel.uuid, channel.name, channel.description, channel.team_uuid
                )
            }
            SuccessCode::InfoThreadFollows(thread) => {
                format!(
                    "242 \"{}\" \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
                    thread.uuid,
                    thread.title,
                    thread.message,
                    thread.creator_uuid,
                    thread.timestamp
                )
            }
            SuccessCode::AvailableCommands => {
                "214 Available commands: LOGIN LOGOUT USERS USER SEND MESSAGES SUBSCRIBE \
                SUBSCRIBED UNSUBSCRIBE USE CREATE LIST INFO\r\n"
                    .to_string()
            }
        }
    }
}

impl From<ErrorCode> for String {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::BadRequest => "400 Bad request\r\n".to_string(),
            ErrorCode::NotFound => "404 Not found\r\n".to_string(),
            ErrorCode::Unauthorized => "403 Forbidden\r\n".to_string(),
            ErrorCode::NoContextSet => "411 No context set\r\n".to_string(),
        }
    }
}

pub fn format_event(event: EventType) -> String {
    match event {
        EventType::UserLoggedIn(event) => format!(
            "EVENT USER_LOGGED_IN \"{}\" \"{}\"\r\n",
            event.user_uuid, event.username
        ),
        EventType::UserLoggedOut(event) => format!(
            "EVENT USER_LOGGED_OUT \"{}\" \"{}\"\r\n",
            event.user_uuid, event.username
        ),
        EventType::TeamCreated(event) => format!(
            "EVENT TEAM_CREATED \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
            event.team_uuid, event.name, event.description, event.creator_uuid
        ),
        EventType::ChannelCreated(event) => format!(
            "EVENT CHANNEL_CREATED \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
            event.channel_uuid, event.name, event.description, event.team_uuid
        ),
        EventType::ThreadCreated(event) => {
            format!(
                "EVENT THREAD_CREATED \"{}\" \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
                event.thread_uuid, event.title, event.body, event.creator_uuid, event.channel_uuid
            )
        }
        EventType::ReplyCreated(event) => format!(
            "EVENT REPLY_CREATED \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
            event.comment_uuid, event.body, event.creator_uuid, event.thread_uuid
        ),
        EventType::MessageReceived(event) => format!(
            "EVENT MESSAGE_RECEIVED \"{}\" \"{}\"\r\n",
            event.sender_uuid, event.body
        ),
        EventType::UserSubscribed(event) => format!(
            "EVENT USER_SUBSCRIBED \"{}\" \"{}\"\r\n",
            event.user_uuid, event.team_uuid
        ),
        EventType::UserUnsubscribed(event) => {
            format!(
                "EVENT TEAM_CREATED \"{}\" \"{}\"\r\n",
                event.user_uuid, event.team_uuid
            )
        }
    }
}

impl EventType {
    pub fn is_global(&self) -> bool {
        matches!(
            self,
            EventType::UserLoggedIn(_) | EventType::UserLoggedOut(_) | EventType::TeamCreated(_)
        )
    }
}

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
    incoming_data_buffer: Vec<u8>,
    pub state: ClientState,

    pub uuid: Option<String>,
    pub context: Option<Context>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        let incoming_data_buffer = Vec::new();
        let state = ClientState::None;

        match stream.set_nonblocking(true) {
            Ok(_) => Self {
                stream,
                incoming_data_buffer,
                state,
                uuid: None,
                context: None,
            },
            Err(e) => {
                println!(
                    "Failed to set client socket non blocking: {}",
                    e.to_string()
                );
                Self {
                    stream,
                    incoming_data_buffer,
                    state,
                    uuid: None,
                    context: None,
                }
            }
        }
    }

    pub fn read(&mut self) -> Result<(), MyTeamsServerError> {
        let mut buffer = [0; 1024];

        loop {
            match self.stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Connexion closed from {:?}", self.stream);
                    self.state = ToBeDisconnected;
                    return Ok(());
                }
                Ok(o) => {
                    self.incoming_data_buffer.extend_from_slice(&buffer[..o]);
                }
                Err(e) if e.kind() == WouldBlock => {
                    return Ok(());
                }
                Err(e) => {
                    return Err(MyTeamsServerError::from(e));
                }
            }
        }
    }

    pub fn write(&mut self, message: impl Into<String>) {
        let message = message.into();

        loop {
            match (self.stream).write_all(message.as_bytes()) {
                Ok(_) => break,
                Err(e) if e.kind() == WouldBlock => {
                    continue;
                }
                Err(e) => {
                    println!(
                        "An error occured during writing the resposes to tcpsocket: {}",
                        e.to_string()
                    );
                    break;
                }
            }
        }
    }

    pub fn get_pending_command(&mut self) -> Option<String> {
        if let Some(pos) = self
            .incoming_data_buffer
            .windows(2)
            .position(|w| w == b"\r\n")
        {
            let line: Vec<u8> = self.incoming_data_buffer.drain(..pos + 2).collect();
            Some(
                String::from_utf8_lossy(&line)
                    .parse()
                    .unwrap_or("FAILED TO EXTRACT ".to_string()),
            )
        } else {
            None
        }
    }

    pub fn send_event(&mut self, event_type: EventType) {
        let formated_event_response = format_event(event_type);

        self.write(formated_event_response);
    }
}
