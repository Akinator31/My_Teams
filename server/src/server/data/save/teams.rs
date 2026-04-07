use crate::server::data::save::MyTeamsSave;
use crate::server::data::team::Team;
use crate::server::data::MyTeamsServerData;
use std::fs::File;

impl MyTeamsServerData {
    pub fn save_teams(&mut self, save_file: &mut File) {
        for team in &self.teams {
            team.save(save_file);
        }
    }

    pub fn load_team(&mut self, args: &[&str]) -> bool {
        let Some(team) = Team::load(args) else {
            println!("The save file is corrupted! The MyTeams server data has been reset.");
            *self = Self::new();
            return false;
        };

        self.teams.push(team);
        true
    }
}
