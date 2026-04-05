use crate::server::data::channel::Channel;
use crate::server::data::user::User;
use crate::utils::uuid::get_uuid;

pub struct Team {
    pub uuid: String,
    pub name: String,
    pub author: String,
    pub description: String,
    pub channels: Vec<Channel>,
    pub subscribed: Vec<User>,
}

pub struct TeamCreatedEvent {
    pub team_uuid: String,
    pub name: String,
    pub description: String,
    pub creator_uuid: String,
}

impl Team {
    pub fn new(name: String, description: String, user_uuid: String) -> Self {
        Team {
            uuid: get_uuid(),
            name,
            author: user_uuid,
            description,
            channels: Vec::new(),
            subscribed: Vec::new(),
        }
    }
}

impl From<(String, String, String, String)> for TeamCreatedEvent {
    fn from(value: (String, String, String, String)) -> Self {
        TeamCreatedEvent {
            team_uuid: value.0,
            name: value.1,
            description: value.2,
            creator_uuid: value.3,
        }
    }
}
