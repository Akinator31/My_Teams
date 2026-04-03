use std::time::{SystemTime, UNIX_EPOCH};

pub mod parsing;
pub mod signals;
pub mod uuid;

pub fn get_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
