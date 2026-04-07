use crate::server::data::save::MyTeamsSave;
use crate::server::data::thread::Reply;
use crate::server::data::MyTeamsServerData;

impl MyTeamsServerData {
    pub fn load_reply(&mut self, args: &[&str]) -> bool {
        println!("REPLY : {:?}", args);

        let Some(reply) = Reply::load(args) else {
            println!("The save file is corrupted! The MyTeams server data has been reset.");
            *self = Self::new();
            return false;
        };

        let Some(team_index) = self.find_teams(reply.team_uuid.clone()) else {
            println!("The save file is corrupted! The MyTeams server data has been reset.");
            *self = Self::new();
            return false;
        };

        let Some(channel_index) = self.find_channels(team_index, reply.channel_uuid.clone()) else {
            println!("The save file is corrupted! The MyTeams server data has been reset.");
            *self = Self::new();
            return false;
        };

        let Some(thread_index) = self.find_threads(team_index, channel_index, reply.thread_uuid.clone()) else {
            println!("The save file is corrupted! The MyTeams server data has been reset.");
            *self = Self::new();
            return false;
        };

        self.teams[team_index].channels[channel_index].threads[thread_index].comments.push(reply);
        true
    }
}