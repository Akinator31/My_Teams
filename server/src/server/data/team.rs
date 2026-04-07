use crate::server::data::channel::Channel;
use crate::server::data::save::MyTeamsSave;
use crate::utils::uuid::get_uuid;
use std::fs::File;
use std::io::Write;

pub struct Team {
    pub uuid: String,
    pub name: String,
    pub author: String,
    pub description: String,
    pub channels: Vec<Channel>,
    pub subscribed: Vec<String>,
}

#[derive(Clone, Debug)]
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

impl MyTeamsSave for Team {
    fn save(&self, save_file: &mut File) {
        match write!(save_file, "TEAM \"{}\" \"{}\" \"{}\" \"{}\" ", self.uuid, self.name, self.author, self.description) {
            Ok(_) => {}
            Err(e) => {
                println!("An error occured while saving a team : {}", e.to_string());
                return;
            }
        }

        for subscribe in &self.subscribed {
            match write!(save_file, "\"{}\" ", subscribe) {
                Ok(_) => {}
                Err(e) => {
                    println!("An error occured while saving a team : {}", e.to_string());
                    return;
                }
            }
        }

        match write!(save_file, "\n") {
            Ok(_) => {}
            Err(e) => {
                println!("An error occured while saving a team : {}", e.to_string());
                return;
            }
        }

        for channel in &self.channels {
            channel.save(save_file);
        }
    }

    fn load(args: &[&str]) -> Option<Self> {
        match args {
            [uuid, name, author, description, subscribed @ ..] => {
                let mut team = Team {
                    uuid: uuid.trim_matches('"').to_string(),
                    name: name.trim_matches('"').to_string(),
                    author: author.trim_matches('"').to_string(),
                    description: description.trim_matches('"').to_string(),
                    channels: Vec::new(),
                    subscribed: Vec::new(),
                };

                for subscribe in subscribed {
                    team.subscribed.push(subscribe.trim_matches('"').to_string());
                }

                Some(team)
            }
            _ => None,
        }
    }
}
