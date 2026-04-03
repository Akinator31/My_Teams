use crate::server::data::user::User;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub mod user;

#[derive(Clone)]
pub struct MyTeamsServerData {
    users: Vec<User>,
    direct_messages: HashMap<UserPair, String>,
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

    pub fn user_exist(&self, user_name: &String) -> Option<usize> {
        for (user_index, user) in self.users.iter().enumerate() {
            if user.user_name == user_name.clone() {
                return Some(user_index);
            }
        }
        None
    }
    pub fn register_send_message(&mut self, pair: UserPair, message: String) {
        self.direct_messages.insert(pair, message);
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
