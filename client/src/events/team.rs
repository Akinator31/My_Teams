use libs::ClientLog;
use crate::utils::parsing::parse_quoted_segments;

pub fn created_team(data: &str) {
    let parts = parse_quoted_segments(data);
    let team_uuid = parts.get(0).cloned().unwrap_or_default();
    let name = parts.get(1).cloned().unwrap_or_default();
    let description = parts.get(2).cloned().unwrap_or_default();
    ClientLog::client_event_team_created(team_uuid, name, description);
}
