use crate::server::data::save::MyTeamsSave;
use crate::server::data::thread::Thread;
use crate::server::data::MyTeamsServerData;

impl MyTeamsServerData {
    pub fn load_thread(&mut self, args: &[&str]) -> bool {
        let Some(thread) = Thread::load(args) else {
            println!("The save file is corrupted! The MyTeams server data has been reset.");
            *self = Self::new();
            return false;
        };

        let Some(team_index) = self.find_teams(thread.team.clone()) else {
            println!("The save file is corrupted! The MyTeams server data has been reset.");
            *self = Self::new();
            return false;
        };

        let Some(channel_index) = self.find_channels(team_index, thread.channel.clone()) else {
            println!("The save file is corrupted! The MyTeams server data has been reset.");
            *self = Self::new();
            return false;
        };

        self.teams[team_index].channels[channel_index].threads.push(thread);
        true
    }
}