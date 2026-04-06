use crate::utils::uuid::get_uuid;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Reply {
    pub uuid: String,
    pub thread_uuid: String,
    pub user_uuid: String,
    pub body: String,
}

pub struct Thread {
    pub channel: String,
    pub uuid: String,
    pub author: String,
    pub title: String,
    pub body: String,
    pub comments: Vec<Reply>,
}

#[derive(Clone, Debug)]
pub struct ThreadCreatedEvent {
    pub thread_uuid: String,
    pub title: String,
    pub body: String,
    pub creator_uuid: String,
    pub channel_uuid: String,
}

#[derive(Clone, Debug)]
pub struct ReplyCreatedEvent {
    pub comment_uuid: String,
    pub body: String,
    pub creator_uuid: String,
    pub thread_uuid: String,
}

impl Thread {
    pub fn new(channel_uuid: String, user_uuid: String, title: String, body: String) -> Self {
        Thread {
            channel: channel_uuid,
            uuid: get_uuid(),
            author: user_uuid,
            title,
            body,
            comments: Vec::new(),
        }
    }
}

impl Reply {
    pub fn new(thread_uuid: String, user_uuid: String, body: String) -> Self {
        Reply {
            uuid: get_uuid(),
            thread_uuid,
            user_uuid,
            body,
        }
    }
}

impl From<(String, String, String, String, String)> for ThreadCreatedEvent {
    fn from(value: (String, String, String, String, String)) -> Self {
        ThreadCreatedEvent {
            thread_uuid: value.0,
            title: value.1,
            body: value.2,
            creator_uuid: value.3,
            channel_uuid: value.4,
        }
    }
}

impl From<(String, String, String, String)> for ReplyCreatedEvent {
    fn from(value: (String, String, String, String)) -> Self {
        ReplyCreatedEvent {
            comment_uuid: value.0,
            body: value.1,
            creator_uuid: value.2,
            thread_uuid: value.3,
        }
    }
}
