use crate::server::data::channel::Channel;
use crate::server::data::user::User;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Comment {
    pub thread_uuid: String,
    pub user_uuid: String,
    pub reply_body: String,
}

pub struct Thread {
    pub channel: Channel,
    pub uuid: String,
    pub author: User,
    pub title: String,
    pub body: String,
    pub comments: Vec<Comment>,
}
