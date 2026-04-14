use libs::ClientLog;

pub fn created_channel(data: &str) {
    let mut parts = data.splitn(4, ' ');
    let channel_uuid = parts.next().unwrap_or("").trim_matches('\"').to_string();
    let name = parts.next().unwrap_or("").trim_matches('\"').to_string();
    let description = parts.next().unwrap_or("").trim_matches('\"').to_string();
    ClientLog::client_event_channel_created(channel_uuid, name, description);
}
