use crate::server::data::thread::Thread;
use crate::utils::uuid::get_uuid;

pub struct Channel {
    pub team: String,
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub threads: Vec<Thread>,
}

pub struct ChannelCreatedEvent {
    pub channel_uuid: String,
    pub name: String,
    pub description: String,
    pub team_uuid: String,
}

impl Channel {
    pub fn new(team_uuid: String, name: String, description: String) -> Self {
        Channel {
            team: team_uuid,
            uuid: get_uuid(),
            name,
            description,
            threads: Vec::new(),
        }
    }
}

impl From<(String, String, String, String)> for ChannelCreatedEvent {
    fn from(value: (String, String, String, String)) -> Self {
        ChannelCreatedEvent {
            channel_uuid: value.0,
            name: value.1,
            description: value.2,
            team_uuid: value.3,
        }
    }
}
