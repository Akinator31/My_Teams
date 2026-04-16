use crate::utils::parsing::parse_quoted_segments;
use libs::ClientLog;

pub fn logout_event(data: &str) {
    let parts = parse_quoted_segments(data);
    let uuid = parts.get(0).cloned().unwrap_or_default();
    let username = parts.get(1).cloned().unwrap_or_default();
    ClientLog::client_event_logged_out(uuid, username);
}
