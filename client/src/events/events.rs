use std::collections::HashMap;

type EventHandlerType = HashMap<String, fn(&str)>;

pub fn events(event: &str) -> EventHandlerType {
    let mut parts = event.splitn(3, ' ');
    let _prefix = parts.next();
    let event_type = parts.next().unwrap_or("");
    let event_data = parts.next().unwrap_or("");
    let mut handlers: EventHandlerType = HashMap::new();

    handlers
}
