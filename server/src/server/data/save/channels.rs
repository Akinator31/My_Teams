use crate::server::data::channel::Channel;
use crate::server::data::save::MyTeamsSave;
use crate::server::data::MyTeamsServerData;

impl MyTeamsServerData {
    pub fn load_channel(&mut self, args: &[&str]) -> bool {
        let Some(channel) = Channel::load(args) else {
            println!("The save file is corrupted! The MyTeams server data has been reset.");
            *self = Self::new();
            return false;
        };

        let Some(team_index) = self.find_teams_by_uuid(channel.team.clone()) else {
            println!("The save file is corrupted! The MyTeams server data has been reset.");
            *self = Self::new();
            return false;
        };

        self.teams[team_index].channels.push(channel);
        true
    }
}