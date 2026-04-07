use crate::server::data::save::MyTeamsSave;
use crate::utils::get_timestamp;
use crate::utils::uuid::get_uuid;
use std::fs::File;
use std::io::Write;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Reply {
    pub uuid: String,
    pub team_uuid: String,
    pub channel_uuid: String,
    pub thread_uuid: String,
    pub user_uuid: String,
    pub body: String,
    pub timestamp: i64,
}

pub struct Thread {
    pub team: String,
    pub channel: String,
    pub uuid: String,
    pub author: String,
    pub title: String,
    pub body: String,
    pub timestamp: i64,
    pub comments: Vec<Reply>,
}

#[derive(Clone, Debug)]
pub struct ThreadCreatedEvent {
    pub thread_uuid: String,
    pub title: String,
    pub body: String,
    pub creator_uuid: String,
    pub channel_uuid: String,
}

#[derive(Clone, Debug)]
pub struct ReplyCreatedEvent {
    pub comment_uuid: String,
    pub body: String,
    pub creator_uuid: String,
    pub thread_uuid: String,
}

impl Thread {
    pub fn new(team_uuid: String, channel_uuid: String, user_uuid: String, title: String, body: String) -> Self {
        Thread {
            team: team_uuid,
            channel: channel_uuid,
            uuid: get_uuid(),
            author: user_uuid,
            title,
            body,
            timestamp: get_timestamp(),
            comments: Vec::new(),
        }
    }
}

impl Reply {
    pub fn new(team_uuid: String, channel_uuid: String, thread_uuid: String, user_uuid: String, body: String) -> Self {
        Reply {
            team_uuid,
            channel_uuid,
            uuid: get_uuid(),
            thread_uuid,
            user_uuid,
            body,
            timestamp: get_timestamp(),
        }
    }
}

impl From<(String, String, String, String, String)> for ThreadCreatedEvent {
    fn from(value: (String, String, String, String, String)) -> Self {
        ThreadCreatedEvent {
            thread_uuid: value.0,
            title: value.1,
            body: value.2,
            creator_uuid: value.3,
            channel_uuid: value.4,
        }
    }
}

impl From<(String, String, String, String)> for ReplyCreatedEvent {
    fn from(value: (String, String, String, String)) -> Self {
        ReplyCreatedEvent {
            comment_uuid: value.0,
            body: value.1,
            creator_uuid: value.2,
            thread_uuid: value.3,
        }
    }
}

impl MyTeamsSave for Thread {
    fn save(&self, save_file: &mut File) {
        match writeln!(save_file, "THREAD \"{}\" \"{}\" \"{}\" \"{}\" \"{}\" \"{}\" \"{}\"", self.team, self.channel, self.uuid, self.author, self.title, self.body, self.timestamp) {
            Ok(_) => {}
            Err(e) => {
                println!("An error occured while saving a team : {}", e.to_string());
                return;
            }
        }

        for reply in &self.comments {
            reply.save(save_file);
        }
    }

    fn load(args: &[&str]) -> Option<Self> {
        match args {
            [team, channel, uuid, author, title, body, timestamp] => {
                Some(Thread {
                    team: team.trim_matches('"').to_string(),
                    channel: channel.trim_matches('"').to_string(),
                    uuid: uuid.trim_matches('"').to_string(),
                    author: author.trim_matches('"').to_string(),
                    title: title.trim_matches('"').to_string(),
                    body: body.trim_matches('"').to_string(),
                    timestamp: body.trim_matches('"').to_string().parse::<i64>().unwrap_or(0),
                    comments: Vec::new(),
                })
            }
            _ => None,
        }
    }
}

struct Replyd {
    pub uuid: String,
    pub team_uuid: String,
    pub channel_uuid: String,
    pub thread_uuid: String,
    pub user_uuid: String,
    pub body: String,
    pub timestamp: i64,
}

impl MyTeamsSave for Reply {
    fn save(&self, save_file: &mut File) {
        match writeln!(save_file, "REPLY \"{}\" \"{}\" \"{}\" \"{}\" \"{}\" \"{}\" \"{}\"", self.uuid, self.team_uuid, self.channel_uuid, self.thread_uuid, self.user_uuid, self.body, self.timestamp) {
            Ok(_) => {}
            Err(e) => {
                println!("An error occured while saving a team : {}", e.to_string());
                return;
            }
        }
    }

    fn load(args: &[&str]) -> Option<Self> {
        match args {
            [uuid, team, channel, thread, user_uuid, body, timestamp] => {
                Some(Reply {
                    uuid: uuid.trim_matches('"').to_string(),
                    team_uuid: team.trim_matches('"').to_string(),
                    channel_uuid: channel.trim_matches('"').to_string(),
                    thread_uuid: thread.trim_matches('"').to_string(),
                    user_uuid: user_uuid.trim_matches('"').to_string(),
                    body: body.trim_matches('"').to_string(),
                    timestamp: timestamp.trim_matches('"').to_string().parse::<i64>().unwrap_or(0),
                })
            }
            _ => None,
        }
    }
}
