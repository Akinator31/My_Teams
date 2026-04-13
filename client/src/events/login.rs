use libs::ClientLog;

pub fn login_event(data: &str) {
    let uuid = data
        .split(' ')
        .nth(0)
        .unwrap_or("")
        .trim_matches('\"')
        .to_string();
    let username = data
        .split(' ')
        .nth(1)
        .unwrap_or("")
        .trim_matches('\"')
        .to_string();
    ClientLog::client_event_logged_in(uuid, username);
}
