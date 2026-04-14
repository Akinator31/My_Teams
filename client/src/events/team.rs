use libs::ClientLog;

pub fn created_team(data: &str) {
    let mut parts = data.splitn(4, ' ');
    let team_uuid = parts.next().unwrap_or("").trim_matches('\"').to_string();
    let name = parts.next().unwrap_or("").trim_matches('\"').to_string();
    let description = parts.next().unwrap_or("").trim_matches('\"').to_string();
    ClientLog::client_event_team_created(team_uuid, name, description);
}
