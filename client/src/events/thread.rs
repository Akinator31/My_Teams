use libs::ClientLog;

// <thread_uuid> <title> <message> <creator_uuid> <time>
pub fn created_thread(data: &str) {
    let mut parts = data.splitn(5, ' ');
    let thread_uuid = parts.next().unwrap_or("").trim_matches('\"').to_string();
    let title = parts.next().unwrap_or("").trim_matches('\"').to_string();
    let message = parts.next().unwrap_or("").trim_matches('\"').to_string();
    let creator_uuid = parts.next().unwrap_or("").trim_matches('\"').to_string();
    let time = parts.next().unwrap_or("").parse::<i64>().unwrap_or(0);
    ClientLog::client_event_thread_created(thread_uuid, creator_uuid, time, title, message);
}

// <team uuid> <thread uuid> <creator uuid> <body>
pub fn thread_reply(data: &str) {
    let mut parts = data.splitn(4, ' ');
    let team_uuid = parts.next().unwrap_or("").trim_matches('\"').to_string();
    let thread_uuid = parts.next().unwrap_or("").trim_matches('\"').to_string();
    let creator_uuid = parts.next().unwrap_or("").trim_matches('\"').to_string();
    let body = parts.next().unwrap_or("").to_string();
    ClientLog::client_event_thread_reply_received(team_uuid, thread_uuid, creator_uuid, body);
}
