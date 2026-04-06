use crate::utils::uuid::get_uuid;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct User {
    pub user_name: String,
    pub uuid: String,
}

#[derive(Clone, Debug)]
pub struct UserLoggedInEvent {
    pub user_uuid: String,
    pub username: String,
}

#[derive(Clone, Debug)]
pub struct UserLoggedOutEvent {
    pub user_uuid: String,
    pub username: String,
}

#[derive(Clone, Debug)]
pub struct UserSubscribedEvent {
    pub user_uuid: String,
    pub team_uuid: String,
}

#[derive(Clone, Debug)]
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

impl From<(String, String)> for UserSubscribedEvent {
    fn from(value: (String, String)) -> Self {
        UserSubscribedEvent {
            user_uuid: value.0,
            team_uuid: value.1,
        }
    }
}

impl From<(String, String)> for UserUnsubscribedEvent {
    fn from(value: (String, String)) -> Self {
        UserUnsubscribedEvent {
            user_uuid: value.0,
            team_uuid: value.1,
        }
    }
}
