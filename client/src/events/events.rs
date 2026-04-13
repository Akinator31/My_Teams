use crate::events::login::login_event;
use crate::events::logout::logout_event;
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

    if let Some(handler) = handlers.get(event_type) {
        handler(event_data);
    } else {
        println!("Unknown event type: {}", event_type);
    }
}
