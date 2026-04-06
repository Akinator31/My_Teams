use crate::server::data::save::MyTeamsSave;
use crate::server::data::{Message, MyTeamsServerData, UserPair};
use crate::utils::parsing::{remove_quoted, split_args};
use std::fs::File;

macro_rules! corrupted_reset {
    ($self:expr) => {
        println!("The save file is corrupted! The MyTeams server data has been reset.");
        *$self = Self::new();
        return false;
    };
}

impl MyTeamsServerData {
    pub fn save_dm(&mut self, save_file: &mut File) {
        self.direct_messages.save(save_file);
    }

    pub fn load_dm(&mut self, args: &[&str]) -> bool {
        let Some(user1) = args.get(0) else {
            corrupted_reset!(self);
        };

        let Some(user2) = args.get(1) else {
            corrupted_reset!(self);
        };

        let Some(messages_raw) = args.get(2) else {
            corrupted_reset!(self);
        };

        let Some(inner) = messages_raw
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
        else {
            corrupted_reset!(self);
        };

        let messages_args = split_args(inner);

        let Some(sender) = messages_args.get(0) else {
            corrupted_reset!(self);
        };
        let Some(body) = messages_args.get(1) else {
            corrupted_reset!(self);
        };
        let Some(timestamp_str) = messages_args.get(2) else {
            corrupted_reset!(self);
        };
        let Ok(timestamp) = remove_quoted(timestamp_str).parse::<i64>() else {
            corrupted_reset!(self);
        };

        let message = Message {
            sender: remove_quoted(&sender.to_string()).to_string(),
            body: remove_quoted(&body.to_string()).to_string(),
            timestamp,
        };

        let pair = UserPair(
            remove_quoted(&user1.to_string()).to_string(),
            remove_quoted(&user2.to_string()).to_string(),
        );

        self.direct_messages.entry(pair).or_default().push(message);

        true
    }
}
