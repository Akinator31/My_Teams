use crate::events::channel::created_channel;
use crate::events::login::login_event;
use crate::events::logout::logout_event;
use crate::events::received::received;
use crate::events::team::created_team;
use crate::events::thread::{created_thread, thread_reply};
use std::collections::HashMap;

type EventHandlerType = HashMap<String, fn(&str)>;

pub fn events(event: &str) {
    let mut parts = event.splitn(3, ' ');
    let _prefix = parts.next();
    let event_type = parts.next().unwrap_or("");
    let event_data = parts.next().unwrap_or("");

    let mut handlers: EventHandlerType = HashMap::new();

    handlers.insert("USER_LOGGED_IN".to_string(), login_event);
    handlers.insert("USER_LOGGED_OUT".to_string(), logout_event);
    handlers.insert("CHANNEL_CREATED".to_string(), created_channel);
    handlers.insert("TEAM_CREATED".to_string(), created_team);
    handlers.insert("MESSAGE_RECEIVED".to_string(), received);
    handlers.insert("THREAD_CREATED".to_string(), created_thread);
    handlers.insert("REPLY_CREATED".to_string(), thread_reply);

    if let Some(handler) = handlers.get(event_type) {
        handler(event_data);
    } else {
        println!("Unknown event type: {}", event_type);
    }
}
