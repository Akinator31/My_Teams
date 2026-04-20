use crate::errors::myteams_errors::MyTeamsServerError;
use crate::errors::myteams_errors::MyTeamsServerError::{
    AlreadyExist, ChannelNotFound, TeamNotFound, ThreadNotFound,
};
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

    pub fn find_teams_by_uuid(&mut self, team_uuid: String) -> Option<usize> {
        for (team_index, team) in self.teams.iter().enumerate() {
            if team.uuid == team_uuid {
                return Some(team_index);
            }
        }
        None
    }

    pub fn find_teams_by_name(&mut self, team_name: String) -> Option<usize> {
        for (team_index, team) in self.teams.iter().enumerate() {
            if team.name == team_name {
                return Some(team_index);
            }
        }
        None
    }

    pub fn find_channels_by_uuid(
        &mut self,
        team_index: usize,
        channel_uuid: String,
    ) -> Option<usize> {
        for (channel_index, channel) in self.teams[team_index].channels.iter().enumerate() {
            if channel.uuid == channel_uuid {
                return Some(channel_index);
            }
        }
        None
    }

    pub fn find_channels_by_name(
        &mut self,
        team_index: usize,
        channel_name: String,
    ) -> Option<usize> {
        for (channel_index, channel) in self.teams[team_index].channels.iter().enumerate() {
            if channel.name == channel_name {
                return Some(channel_index);
            }
        }
        None
    }

    pub fn find_threads_by_uuid(
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

    pub fn find_threads_by_name(
        &mut self,
        team_index: usize,
        channel_index: usize,
        thread_title: String,
    ) -> Option<usize> {
        for (thread_index, thread) in self.teams[team_index].channels[channel_index]
            .threads
            .iter()
            .enumerate()
        {
            if thread.title == thread_title {
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
    ) -> Result<(String, usize), MyTeamsServerError> {
        let new_team = Team::new(team_name.clone(), team_description, client_uuid);

        if self.find_teams_by_name(team_name).is_some() {
            return Err(AlreadyExist);
        }

        let new_team_uuid = new_team.uuid.clone();
        let new_team_index = {
            self.teams.push(new_team);
            self.teams.len() - 1
        };

        Ok((new_team_uuid, new_team_index))
    }

    pub fn create_channel(
        &mut self,
        team_uuid: String,
        channel_name: String,
        channel_description: String,
    ) -> Result<(String, usize), MyTeamsServerError> {
        let new_channel =
            Channel::new(team_uuid.clone(), channel_name.clone(), channel_description);

        let Some(team_index) = self.find_teams_by_uuid(team_uuid.clone()) else {
            return Err(TeamNotFound(team_uuid));
        };

        if self
            .find_channels_by_name(team_index, channel_name)
            .is_some()
        {
            return Err(AlreadyExist);
        }

        let new_channel_uuid = new_channel.uuid.clone();
        let new_channel_index = {
            self.teams[team_index].channels.push(new_channel);
            self.teams[team_index].channels.len() - 1
        };

        Ok((new_channel_uuid, new_channel_index))
    }

    pub fn create_thread(
        &mut self,
        team_uuid: String,
        user_uuid: String,
        channel_uuid: String,
        thread_title: String,
        thread_body: String,
    ) -> Result<(String, usize, i64), MyTeamsServerError> {
        let new_thread = Thread::new(
            team_uuid.clone(),
            channel_uuid.clone(),
            user_uuid.clone(),
            thread_title.clone(),
            thread_body.clone(),
        );

        let Some(team_index) = self.find_teams_by_uuid(team_uuid.clone()) else {
            return Err(TeamNotFound(team_uuid));
        };
        let Some(channel_index) = self.find_channels_by_uuid(team_index, channel_uuid.clone())
        else {
            return Err(ChannelNotFound(channel_uuid));
        };

        if self
            .find_threads_by_name(team_index, channel_index, thread_title)
            .is_some()
        {
            return Err(AlreadyExist);
        }

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

        Ok((new_thread_uuid, new_thread_index, new_thread_timestamp))
    }

    pub fn create_reply(
        &mut self,
        team_uuid: String,
        user_uuid: String,
        channel_uuid: String,
        thread_uuid: String,
        body: String,
    ) -> Result<(String, usize, i64), MyTeamsServerError> {
        let new_reply = Reply::new(
            team_uuid.clone(),
            channel_uuid.clone(),
            thread_uuid.clone(),
            user_uuid,
            body,
        );

        let Some(team_index) = self.find_teams_by_uuid(team_uuid.clone()) else {
            return Err(TeamNotFound(team_uuid));
        };
        let Some(channel_index) = self.find_channels_by_uuid(team_index, channel_uuid.clone())
        else {
            return Err(ChannelNotFound(channel_uuid));
        };
        let Some(thread_index) =
            self.find_threads_by_uuid(team_index, channel_index, thread_uuid.clone())
        else {
            return Err(ThreadNotFound(thread_uuid));
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

        Ok((new_reply_uuid, new_reply_index, new_reply_timestamp))
    }

    pub fn subscribe_to_team(&mut self, team_uuid: String, user_uuid: String) -> bool {
        let Some(team_index) = self.find_teams_by_uuid(team_uuid) else {
            return false;
        };

        if !self.teams[team_index].subscribed.contains(&user_uuid) {
            self.teams[team_index].subscribed.push(user_uuid);
        }

        true
    }

    pub fn unsubscribe_from_team(&mut self, team_uuid: String, user_uuid: String) -> bool {
        let Some(team_index) = self.find_teams_by_uuid(team_uuid) else {
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
    use crate::server::data::{MyTeamsServerData, UserPair};
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

    #[test]
    fn test_user_creation() {
        let mut data = MyTeamsServerData::new();
        let name = "Alice".to_string();
        let user = data.create_user(&name);
        assert_eq!(user.user_name, name);
        assert!(data.user_exist_by_name(&name).is_some());
        assert!(data.user_exist_by_uuid(&user.uuid).is_some());
    }

    #[test]
    fn test_team_creation() {
        let mut data = MyTeamsServerData::new();
        let creator_uuid = "user-uuid".to_string();
        let name = "Team1".to_string();
        let desc = "Description".to_string();

        let result = data.create_team(creator_uuid.clone(), name.clone(), desc.clone());
        assert!(result.is_ok());
        let (uuid, index) = result.unwrap();
        assert_eq!(data.teams[index].name, name);
        assert_eq!(data.teams[index].uuid, uuid);

        let result2 = data.create_team(creator_uuid, name, desc);
        assert!(result2.is_err());
    }

    #[test]
    fn test_channel_creation() {
        let mut data = MyTeamsServerData::new();
        let team_uuid = data
            .create_team("user".into(), "T1".into(), "D1".into())
            .unwrap()
            .0;

        let result = data.create_channel(team_uuid.clone(), "C1".into(), "CD1".into());
        assert!(result.is_ok());
        let (channel_uuid, index) = result.unwrap();
        assert_eq!(data.teams[0].channels[index].uuid, channel_uuid);

        let result_err = data.create_channel("wrong-team".into(), "C2".into(), "CD2".into());
        assert!(result_err.is_err());
    }

    #[test]
    fn test_thread_and_reply_creation() {
        let mut data = MyTeamsServerData::new();
        let team_uuid = data
            .create_team("user".into(), "T1".into(), "D1".into())
            .unwrap()
            .0;
        let channel_uuid = data
            .create_channel(team_uuid.clone(), "C1".into(), "CD1".into())
            .unwrap()
            .0;

        let thread_result = data.create_thread(
            team_uuid.clone(),
            "user".into(),
            channel_uuid.clone(),
            "Title".into(),
            "Body".into(),
        );
        assert!(thread_result.is_ok());
        let (thread_uuid, thread_index, _) = thread_result.unwrap();

        assert_eq!(
            data.teams[0].channels[0].threads[thread_index]
                .comments
                .len(),
            1
        );

        let reply_result = data.create_reply(
            team_uuid,
            "user".into(),
            channel_uuid,
            thread_uuid,
            "New Reply".into(),
        );
        assert!(reply_result.is_ok());
        assert_eq!(
            data.teams[0].channels[0].threads[thread_index]
                .comments
                .len(),
            2
        );
    }
}
