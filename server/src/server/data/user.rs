use crate::server::data::save::MyTeamsSave;
use crate::utils::uuid::get_uuid;
use std::fs::File;
use std::io::Write;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct User {
    pub user_name: String,
    pub uuid: String,
}

#[derive(Clone, Debug)]
pub struct UserLoggedInEvent {
    pub user_uuid: String,
    pub username: String,
}

#[derive(Clone, Debug)]
pub struct UserLoggedOutEvent {
    pub user_uuid: String,
    pub username: String,
}

#[derive(Clone, Debug)]
pub struct UserSubscribedEvent {
    pub user_uuid: String,
    pub team_uuid: String,
}

#[derive(Clone, Debug)]
pub struct UserUnsubscribedEvent {
    pub user_uuid: String,
    pub team_uuid: String,
}

impl User {
    pub fn new(user_name: String) -> Self {
        Self {
            user_name,
            uuid: get_uuid(),
        }
    }
}

impl From<(String, String)> for UserLoggedInEvent {
    fn from(value: (String, String)) -> Self {
        UserLoggedInEvent {
            user_uuid: value.0,
            username: value.1,
        }
    }
}

impl From<(String, String)> for UserLoggedOutEvent {
    fn from(value: (String, String)) -> Self {
        UserLoggedOutEvent {
            user_uuid: value.0,
            username: value.1,
        }
    }
}

impl From<(String, String)> for UserSubscribedEvent {
    fn from(value: (String, String)) -> Self {
        UserSubscribedEvent {
            user_uuid: value.0,
            team_uuid: value.1,
        }
    }
}

impl From<(String, String)> for UserUnsubscribedEvent {
    fn from(value: (String, String)) -> Self {
        UserUnsubscribedEvent {
            user_uuid: value.0,
            team_uuid: value.1,
        }
    }
}

impl MyTeamsSave for User {
    fn save(&self, save_file: &mut File) {
        match writeln!(save_file, "USER \"{}\" \"{}\"", self.user_name, self.uuid) {
            Ok(_) => {}
            Err(e) => {
                println!("An error occured while saving a user : {}", e.to_string());
                return;
            }
        }
    }

    fn load(args: &[&str]) -> Option<Self> {
        match args {
            [username, uuid] => Some(User {
                user_name: username.trim_matches('"').to_string(),
                uuid: uuid.trim_matches('"').to_string(),
            }),
            _ => None,
        }
    }
}
