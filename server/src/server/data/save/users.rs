use crate::server::data::save::MyTeamsSave;
use crate::server::data::user::User;
use crate::server::data::MyTeamsServerData;
use std::fs::File;

impl MyTeamsServerData {
    pub fn save_users(&mut self, save_file: &mut File) {
        for user in &self.users {
            user.save(save_file)
        }
    }

    pub fn load_user(&mut self, args: &[&str]) -> bool {
        let Some(user) = User::load(args) else {
            println!("The save file is corrupted! The MyTeams server data has been reset.");
            *self = Self::new();
            return false;
        };

        self.users.push(user);
        true
    }
}
