use crate::server::data::team::Team;
use crate::server::data::thread::Thread;

pub struct Channel {
    pub team: Team,
    pub uuid: String,
    pub name: String,
    pub threads: Vec<Thread>,
}
