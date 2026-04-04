use crate::server::data::channel::Channel;
use crate::server::data::user::User;

pub struct Team {
    pub uuid: String,
    pub name: String,
    pub author: User,
    pub channels: Vec<Channel>,
}
