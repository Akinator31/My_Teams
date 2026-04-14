use crate::server::data::channel::Channel;
use crate::server::data::save::MyTeamsSave;
use crate::server::data::team::Team;
use crate::server::data::thread::{Reply, Thread};
use crate::server::data::user::User;
use crate::utils::get_timestamp;
use std::cmp::PartialEq;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;

pub mod channel;
pub mod save;
pub mod team;
pub mod thread;
pub mod user;

type DirectMessages = HashMap<UserPair, Vec<Message>>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Message {
    pub sender: String,
    pub timestamp: i64,
    pub body: String,
}

#[derive(Clone, Debug)]
pub struct MessageCreatedEvent {
    pub sender_uuid: String,
    pub body: String,
}

pub struct MyTeamsServerData {
    pub users: Vec<User>,
    direct_messages: DirectMessages,
    pub teams: Vec<Team>,
}

#[derive(Clone, Eq)]
pub struct UserPair(pub String, pub String);

impl PartialEq for UserPair {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl From<(String, String)> for MessageCreatedEvent {
    fn from(value: (String, String)) -> Self {
        MessageCreatedEvent {
            sender_uuid: value.0,
            body: value.1,
        }
    }
}

impl Hash for UserPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0 < self.1 {
            self.0.hash(state);
            self.1.hash(state);
        } else {
            self.1.hash(state);
            self.0.hash(state);
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[\"{}\" \"{}\" \"{}\"]",
            self.sender, self.body, self.timestamp
        )
    }
}

impl MyTeamsServerData {
    pub fn new() -> Self {
        MyTeamsServerData {
            users: Vec::new(),
            direct_messages: HashMap::new(),
            teams: Vec::new(),
        }
    }

    pub fn get_user_by_uuid(&mut self, user_uuid: &String) -> Option<&User> {
        for user in &self.users {
            if user.uuid == user_uuid.clone() {
                return Some(user);
            }
        }
        None
    }

    pub fn create_user(&mut self, user_name: &String) -> User {
        let new_user = User::new(user_name.clone());

        self.users.push(new_user.clone());
        new_user
    }

    pub fn user_exist_by_name(&self, user_name: &String) -> Option<usize> {
        for (user_index, user) in self.users.iter().enumerate() {
            if user.user_name == user_name.clone() {
                return Some(user_index);
            }
        }
        None
    }

    pub fn user_exist_by_uuid(&self, uuid: &String) -> Option<usize> {
        for (user_index, user) in self.users.iter().enumerate() {
            if user.uuid == uuid.clone() {
                return Some(user_index);
            }
        }
        None
    }

    pub fn register_send_message(&mut self, pair: UserPair, message: String) {
        match self.direct_messages.entry(pair.clone()) {
            Entry::Vacant(e) => {
                e.insert(vec![Message {
                    sender: pair.0,
                    timestamp: get_timestamp(),
                    body: message,
                }]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(Message {
                    sender: pair.0,
                    timestamp: get_timestamp(),
                    body: message,
                });
            }
        }
    }

    pub fn get_messages(&mut self, pair: UserPair) -> Option<Vec<Message>> {
        for (user_pair, messages) in &self.direct_messages {
            if *user_pair == pair {
                return Some(messages.clone());
            }
        }
        None
    }

    pub fn find_teams(&mut self, team_uuid: String) -> Option<usize> {
        for (team_index, team) in self.teams.iter().enumerate() {
            if team.uuid == team_uuid {
                return Some(team_index);
            }
        }
        None
    }

    pub fn find_channels(&mut self, team_index: usize, channel_uuid: String) -> Option<usize> {
        for (channel_index, channel) in self.teams[team_index].channels.iter().enumerate() {
            if channel.uuid == channel_uuid {
                return Some(channel_index);
            }
        }
        None
    }

    pub fn find_threads(
        &mut self,
        team_index: usize,
        channel_index: usize,
        thread_uuid: String,
    ) -> Option<usize> {
        for (thread_index, thread) in self.teams[team_index].channels[channel_index]
            .threads
            .iter()
            .enumerate()
        {
            if thread.uuid == thread_uuid {
                return Some(thread_index);
            }
        }
        None
    }

    pub fn create_team(
        &mut self,
        client_uuid: String,
        team_name: String,
        team_description: String,
    ) -> (String, usize) {
        let new_team = Team::new(team_name, team_description, client_uuid);

        let new_team_uuid = new_team.uuid.clone();
        let new_team_index = {
            self.teams.push(new_team);
            self.teams.len() - 1
        };

        (new_team_uuid, new_team_index)
    }

    pub fn create_channel(
        &mut self,
        team_uuid: String,
        channel_name: String,
        channel_description: String,
    ) -> Option<(String, usize)> {
        let new_channel = Channel::new(team_uuid.clone(), channel_name, channel_description);

        let Some(team_index) = self.find_teams(team_uuid) else {
            return None;
        };

        let new_channel_uuid = new_channel.uuid.clone();
        let new_channel_index = {
            self.teams[team_index].channels.push(new_channel);
            self.teams[team_index].channels.len() - 1
        };

        Some((new_channel_uuid, new_channel_index))
    }

    pub fn create_thread(
        &mut self,
        team_uuid: String,
        user_uuid: String,
        channel_uuid: String,
        thread_title: String,
        thread_body: String,
    ) -> Option<(String, usize, i64)> {
        let new_thread = Thread::new(
            team_uuid.clone(),
            channel_uuid.clone(),
            user_uuid.clone(),
            thread_title,
            thread_body.clone(),
        );

        let Some(team_index) = self.find_teams(team_uuid.clone()) else {
            return None;
        };
        let Some(channel_index) = self.find_channels(team_index, channel_uuid.clone()) else {
            return None;
        };

        let new_thread_uuid = new_thread.uuid.clone();
        let new_thread_timestamp = new_thread.timestamp.clone();
        let new_thread_index = {
            self.teams[team_index].channels[channel_index]
                .threads
                .push(new_thread);
            self.teams[team_index].channels[channel_index].threads.len() - 1
        };

        self.teams[team_index].channels[channel_index].threads[new_thread_index]
            .comments
            .push(Reply::new(
                team_uuid,
                channel_uuid,
                new_thread_uuid.clone(),
                user_uuid,
                thread_body,
            ));

        Some((new_thread_uuid, new_thread_index, new_thread_timestamp))
    }

    pub fn create_reply(
        &mut self,
        team_uuid: String,
        user_uuid: String,
        channel_uuid: String,
        thread_uuid: String,
        body: String,
    ) -> Option<(String, usize, i64)> {
        let new_reply = Reply::new(
            team_uuid.clone(),
            channel_uuid.clone(),
            thread_uuid.clone(),
            user_uuid,
            body,
        );

        let Some(team_index) = self.find_teams(team_uuid) else {
            return None;
        };
        let Some(channel_index) = self.find_channels(team_index, channel_uuid) else {
            return None;
        };
        let Some(thread_index) = self.find_threads(team_index, channel_index, thread_uuid) else {
            return None;
        };

        let new_reply_uuid = new_reply.uuid.clone();
        let new_reply_timestamp = new_reply.timestamp;
        let new_reply_index = {
            self.teams[team_index].channels[channel_index].threads[thread_index]
                .comments
                .push(new_reply);
            self.teams[team_index].channels[channel_index].threads[thread_index]
                .comments
                .len()
                - 1
        };

        Some((new_reply_uuid, new_reply_index, new_reply_timestamp))
    }

    pub fn subscribe_to_team(&mut self, team_uuid: String, user_uuid: String) -> bool {
        let Some(team_index) = self.find_teams(team_uuid) else {
            return false;
        };

        if !self.teams[team_index].subscribed.contains(&user_uuid) {
            self.teams[team_index].subscribed.push(user_uuid);
        }

        true
    }

    pub fn unsubscribe_from_team(&mut self, team_uuid: String, user_uuid: String) -> bool {
        let Some(team_index) = self.find_teams(team_uuid) else {
            return false;
        };

        self.teams[team_index]
            .subscribed
            .retain(|c| c.clone() != user_uuid);

        true
    }
}

impl MyTeamsSave for DirectMessages {
    fn save(&self, save_file: &mut File) {
        for (user_pair, messages) in self {
            for message in messages {
                match writeln!(
                    save_file,
                    "DIRECT_MESSAGES \"{}\" \"{}\" {}",
                    user_pair.0,
                    user_pair.1,
                    message.clone()
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("An error occured while saving a user : {}", e.to_string());
                        return;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::server::data::UserPair;
    use std::collections::HashMap;

    #[test]
    fn valid_hash() {
        let mut hashmap: HashMap<UserPair, String> = HashMap::new();
        let pair1 = UserPair(String::from("Hello"), String::from("HelloDouble"));
        let pair2 = UserPair(String::from("HelloDouble"), String::from("Hello"));

        hashmap.insert(pair1.clone(), "default".to_string());
        assert!(hashmap.contains_key(&pair1));
        assert!(hashmap.contains_key(&pair2));
    }
}
