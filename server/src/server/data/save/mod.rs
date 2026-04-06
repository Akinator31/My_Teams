use crate::server::data::MyTeamsServerData;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod dm;
pub mod teams;
pub mod users;

pub trait MyTeamsSave: Sized {
    fn save(&self, save_file: &mut File);

    fn load(args: &[&str]) -> Option<Self>;
}

impl MyTeamsServerData {
    pub fn save(&mut self) {
        let mut save_file = match File::create("save.myteams") {
            Ok(f) => f,
            Err(e) => {
                println!("Failed to save MyTeams server : {}", e.to_string());
                return;
            }
        };

        self.save_users(&mut save_file);
        self.save_dm(&save_file);
        self.save_teams(&save_file);
    }

    pub fn load_save(&mut self) {
        let save_file = match File::open("save.myteams") {
            Ok(f) => f,
            Err(_) => {
                println!("No save file. Starting MyTeams server...");
                return;
            }
        };

        let reader = BufReader::new(save_file);

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => {
                    println!("The save file is corrupted! The MyTeams server data has been reset.");
                    *self = Self::new();
                    return;
                }
            };

            match line.split(' ').collect::<Vec<&str>>().as_slice() {
                ["USER", args @ ..] if self.load_user(args) => Self::logging_load("USER", args),
                _ => {}
            }
        }
    }

    fn logging_load(element: &str, args: &[&str]) {
        match (element, args) {
            ("USER", _) => println!("LOADED {} WITH FOLLOWING DATA : {:?}", element, args),
            _ => {}
        }
    }
}
