use crate::clients::client::Client;

fn check_command_format(command_args: String) -> Option<String> {
    if command_args.split(" ").count() > 1 {
        return None;
    }
    Some(command_args.split("").next().unwrap_or("").to_string())
}

pub fn login(client: &mut Client, command_args: String) -> bool {
    if let Some(user_name) = check_command_format(command_args) {
        println!("username : {}", user_name);
        libs::ServerLog::server_event_user_logged_in("tkt".to_string());
        true
    } else {
        false
    }
}