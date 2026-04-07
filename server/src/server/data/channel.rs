use crate::server::data::save::MyTeamsSave;
use crate::server::data::thread::Thread;
use crate::utils::uuid::get_uuid;
use std::fs::File;
use std::io::Write;

pub struct Channel {
    pub team: String,
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub threads: Vec<Thread>,
}

#[derive(Clone, Debug)]
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

impl MyTeamsSave for Channel {
    fn save(&self, save_file: &mut File) {
        match writeln!(save_file, "CHANNEL \"{}\" \"{}\" \"{}\" \"{}\"", self.team, self.uuid, self.name, self.description) {
            Ok(_) => {}
            Err(e) => {
                println!("An error occurred while saving a user : {}", e.to_string());
                return;
            }
        }

        for thread in &self.threads {
            thread.save(save_file);
        }
    }

    fn load(args: &[&str]) -> Option<Self> {
        match args {
            [team, uuid, name, description] => {
                Some(Channel {
                    team: team.trim_matches('"').to_string(),
                    uuid: uuid.trim_matches('"').to_string(),
                    name: name.trim_matches('"').to_string(),
                    description: description.trim_matches('"').to_string(),
                    threads: Vec::new(),
                })
            }
            _ => None,
        }
    }
}
