use crate::server::data::user::User;

pub mod user;

pub struct MyTeamsServerData {
    users: Vec<User>,
}

impl MyTeamsServerData {
    pub fn new() -> Self {
        MyTeamsServerData { users: Vec::new() }
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

        self.users.push(User::new(user_name.clone()));
        new_user
    }

    pub fn user_exist(&self, user_name: &String) -> bool {
        for user in &self.users {
            if user.user_name == user_name.clone() {
                return true;
            }
        }
        false
    }
}