use crate::clients::client::ClientState::ToBeDisconnected;
use crate::errors::myteams_errors::MyTeamsServerError;
use crate::server::data::user::User;
use crate::server::data::Message;
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
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SuccessCode {
    Okay(OkeyResponse),
    UserLoggedIn(String),
    UserLoggedOut,
    MessageSent,
    MessageListFollows(Message),
    UsersListFollows(User, bool),
    UserInfoFollows(User, bool),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    BadRequest,
    NotFound,
    Unauthorized,
}

pub enum EventType {
    UserLoggedIn(String, String),
    UserLoggedOut(String, String),
    TeamCreated(String, String, String, String),
    ChannelCreated(String, String, String, String),
    ThreadCreated(String, String, String, String, String),
    ReplyCreated(String, String, String, String),
    MessageReceived(String, String),
    UserSubscribed(String, String),
    UserUnsubscribed(String, String),
}

impl Display for OkeyResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OkeyResponse::Connected => write!(f, "Connected to MyTeams server"),
            OkeyResponse::EndOfMessages => write!(f, "End of messages"),
            OkeyResponse::EndOfUsers => write!(f, "End of users list"),
        }
    }
}

impl From<SuccessCode> for String {
    fn from(value: SuccessCode) -> Self {
        match value {
            SuccessCode::Okay(message) => format!("200 {}\r\n", message),
            SuccessCode::UserLoggedIn(username) => {
                format!("210 User logged in. UUID: {}\r\n", username)
            }
            SuccessCode::UserLoggedOut => "211 User logged out.\r\n".to_string(),
            SuccessCode::MessageSent => "220 Message sent\r\n".to_string(),
            SuccessCode::MessageListFollows(message) => {
                format!(
                    "221 \"{}\" \"{}\" \"{}\"\r\n",
                    message.sender, message.timestamp, message.body
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
        }
    }
}

impl From<ErrorCode> for String {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::BadRequest => "400 Bad request\r\n".to_string(),
            ErrorCode::NotFound => "404 Not found\r\n".to_string(),
            ErrorCode::Unauthorized => "403 Forbidden (insufficient permissions)\r\n".to_string(),
        }
    }
}

pub fn format_event(event: EventType) -> String {
    match event {
        EventType::UserLoggedIn(user_uuid, username) => format!(
            "EVENT USER_LOGGED_IN \"{}\" \"{}\"\r\n",
            user_uuid, username
        ),
        EventType::UserLoggedOut(user_uuid, username) => format!(
            "EVENT USER_LOGGED_OUT \"{}\" \"{}\"\r\n",
            user_uuid, username
        ),
        EventType::TeamCreated(team_uuid, name, description, creator_uuid) => format!(
            "EVENT TEAM_CREATED \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
            team_uuid, name, description, creator_uuid
        ),
        EventType::ChannelCreated(channel_uuid, name, description, team_uuid) => format!(
            "EVENT CHANNEL_CREATED \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
            channel_uuid, name, description, team_uuid
        ),
        EventType::ThreadCreated(thread_uuid, title, message, creator_uuid, channel_uuid) => {
            format!(
                "EVENT THREAD_CREATED \"{}\" \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
                thread_uuid, title, message, creator_uuid, channel_uuid
            )
        }
        EventType::ReplyCreated(comment_uuid, body, creator_uuid, thread_uuid) => format!(
            "EVENT REPLY_CREATED \"{}\" \"{}\" \"{}\" \"{}\"\r\n",
            comment_uuid, body, creator_uuid, thread_uuid
        ),
        EventType::MessageReceived(sender_uuid, body) => format!(
            "EVENT MESSAGE_RECEIVED \"{}\" \"{}\"\r\n",
            sender_uuid, body
        ),
        EventType::UserSubscribed(user_uuid, team_uuid) => format!(
            "EVENT USER_SUBSCRIBED \"{}\" \"{}\"\r\n",
            user_uuid, team_uuid
        ),
        EventType::UserUnsubscribed(user_uuid, team_uuid) => {
            format!("EVENT TEAM_CREATED \"{}\" \"{}\"\r\n", user_uuid, team_uuid)
        }
    }
}

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
    incoming_data_buffer: Vec<u8>,
    pub state: ClientState,

    pub uuid: Option<String>,
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
