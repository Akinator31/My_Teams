use crate::server::data::user::User;
use crate::utils::get_timestamp;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub mod user;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Message {
    pub sender: String,
    pub timestamp: i64,
    pub body: String,
}

#[derive(Clone)]
pub struct MyTeamsServerData {
    pub users: Vec<User>,
    direct_messages: HashMap<UserPair, Vec<Message>>,
}

#[derive(Clone, Eq)]
pub struct UserPair(pub String, pub String);

impl PartialEq for UserPair {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
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

impl MyTeamsServerData {
    pub fn new() -> Self {
        MyTeamsServerData {
            users: Vec::new(),
            direct_messages: HashMap::new(),
        }
    }

    pub fn get_user(&mut self, user_name: &String) -> Option<&User> {
        for user in &self.users {
            if user.user_name == user_name.clone() {
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
