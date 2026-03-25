mod bindings;

fn main() {
    println!("Hello, world!");

    libs::ServerLog::server_event_team_created("ff4c8d30-5425-40b2-b5ff-594dac395a14".to_string(), "Team Test".to_string(), "2a65796e-0c66-4dd4-bda1-56eb28392195".to_string());
}
