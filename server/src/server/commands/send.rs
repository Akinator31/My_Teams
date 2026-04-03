use crate::clients::client::EventType::MessageReceived;
use crate::server::data::UserPair;
use crate::server::server::MyTeamsServer;

fn parse_quoted_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut chars = input.trim().chars().peekable();

    while chars.peek().is_some() {
        while chars.peek() == Some(&' ') {
            chars.next();
        }

        match chars.peek() {
            Some(&'"') => {
                let mut token = String::default();
                let mut closed = false;
                chars.next();

                for c in chars.by_ref() {
                    if c == '"' { closed = true; break }
                    token.push(c);
                }
                if closed != true { continue }
                args.push(token);
            }
            Some(_) => {
                chars.next();
            }
            None => break
        }
    }

    args
}

fn check_command_format(command_args: &str) -> Option<(String, String)> {
    let args = parse_quoted_args(command_args);

    if args.len() == 2 {
        Some((args[0].clone(), args[1].clone()))
    } else {
        None
    }
}

pub fn send(server: &mut MyTeamsServer, client_index: usize, command_args: String) -> bool {
    let client_uuid = match server.client_manager.clients[client_index].uuid.clone() {
        Some(uuid) => uuid,
        None => {
            println!("User not connected!");
            return false;
        }
    };

    let Some(args) = check_command_format(&command_args) else {
        return false;
    };

    let receiver_uuid = &args.0.to_string();
    let message = &args.1.to_string();

    let receiver_index = match server.client_manager.is_client_logged_in(receiver_uuid.clone()) {
        Some(idx) => idx,
        None => return false
    };

    let users_pair = UserPair (client_uuid.clone(), receiver_uuid.clone() );

    server.client_manager.clients[receiver_index]
        .send_event(MessageReceived(client_uuid, message.clone()));

    server.data.register_send_message(users_pair, message.clone());

    true
}