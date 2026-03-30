use crate::utils::uuid::get_uuid;

#[derive(Clone)]
pub struct User {
    pub user_name: String,
    pub uuid: String,
}

impl User {
    pub fn new(user_name: String) -> Self {
        Self {user_name, uuid: get_uuid() }
    }
}