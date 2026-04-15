use libs::ClientLog;

pub fn received(data: &str) {
    let mut parts = data.split('"').filter(|s| !s.trim().is_empty());
    let uuid = parts.next().unwrap_or("").trim_matches('\"').to_string();
    let body = parts.next().unwrap_or("").trim_matches('\"').to_string();
    ClientLog::client_event_private_message_received(uuid, body);
}
