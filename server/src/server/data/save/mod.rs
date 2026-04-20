use crate::server::data::MyTeamsServerData;
use crate::utils::parsing::split_args;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod dm;
pub mod teams;
pub mod users;
mod channels;
mod threads;
mod replies;

pub trait MyTeamsSave: Sized {
    fn save(&self, save_file: &mut File);

    fn load(_args: &[&str]) -> Option<Self> {
        None
    }
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
        self.save_dm(&mut save_file);
        self.save_teams(&mut save_file);
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

            match split_args(&line).as_slice() {
                ["USER", args @ ..] if self.load_user(args) => {
                    let uuid = args.get(0).map(|s| s.trim_matches('"')).unwrap_or("");
                    let name = args.get(1).map(|s| s.trim_matches('"')).unwrap_or("");
                    libs::ServerLog::server_event_user_loaded(uuid.to_string(), name.to_string());
                    Self::logging_load("USER", args);
                }
                ["DIRECT_MESSAGES", args @ ..] if self.load_dm(args) => {
                    Self::logging_load("DIRECT_MESSAGES", args)
                }
                ["TEAM", args @ ..] if self.load_team(args) => Self::logging_load("TEAM", args),
                ["CHANNEL", args @ ..] if self.load_channel(args) => Self::logging_load("CHANNEL", args),
                ["THREAD", args @ ..] if self.load_thread(args) => Self::logging_load("THREAD", args),
                ["REPLY", args @ ..] if self.load_reply(args) => Self::logging_load("REPLY", args),
                _ => {
                    println!("The save file is corrupted! The MyTeams server data has been reset.");
                    *self = Self::new();
                    return;
                }
            }
        }
    }

    fn logging_load(element: &str, args: &[&str]) {
        match (element, args) {
            ("USER", _) => println!("LOADED {} WITH FOLLOWING DATA : {:?}", element, args),
            ("DIRECT_MESSAGES", _) => {
                println!("LOADED {} WITH FOLLOWING DATA : {:?}", element, args)
            }
            ("TEAM", _) => {
                println!("LOADED {} WITH FOLLOWING DATA : {:?}", element, args)
            }
            ("CHANNEL", _) => {
                println!("LOADED {} WITH FOLLOWING DATA : {:?}", element, args)
            }
            ("THREAD", _) => {
                println!("LOADED {} WITH FOLLOWING DATA : {:?}", element, args)
            }
            ("REPLY", _) => {
                println!("LOADED {} WITH FOLLOWING DATA : {:?}", element, args)
            }
            _ => {}
        }
    }
}
