use crate::utils::uuid::get_uuid;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct User {
    pub user_name: String,
    pub uuid: String,
}

pub struct UserLoggedInEvent {
    pub user_uuid: String,
    pub username: String,
}

pub struct UserLoggedOutEvent {
    pub user_uuid: String,
    pub username: String,
}

pub struct UserSubscribedEvent {
    pub user_uuid: String,
    pub team_uuid: String,
}

pub struct UserUnsubscribedEvent {
    pub user_uuid: String,
    pub team_uuid: String,
}

impl User {
    pub fn new(user_name: String) -> Self {
        Self {
            user_name,
            uuid: get_uuid(),
        }
    }
}
