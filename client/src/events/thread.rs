use libs::ClientLog;
use crate::utils::parsing::parse_quoted_segments;

// <thread_uuid> <title> <message> <creator_uuid> <time>
pub fn created_thread(data: &str) {
    let parts = parse_quoted_segments(data);
    let thread_uuid = parts.get(0).cloned().unwrap_or_default();
    let title = parts.get(1).cloned().unwrap_or_default();
    let message = parts.get(2).cloned().unwrap_or_default();
    let creator_uuid = parts.get(3).cloned().unwrap_or_default();
    let time = parts.get(4).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    ClientLog::client_event_thread_created(thread_uuid, creator_uuid, time, title, message);
}

// <team uuid> <message uuid> <body> <creator uuid> <thread uuid>
pub fn thread_reply(data: &str) {
    let parts = parse_quoted_segments(data);
    let team_uuid = parts.get(0).cloned().unwrap_or_default();
    let thread_uuid = parts.get(1).cloned().unwrap_or_default();
    let body = parts.get(2).cloned().unwrap_or_default();
    let creator_uuid = parts.get(3).cloned().unwrap_or_default();
    ClientLog::client_event_thread_reply_received(team_uuid, thread_uuid, creator_uuid, body);
}
