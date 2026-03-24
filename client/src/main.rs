mod bindings;

use std::ffi::CString;
use std::io::BufRead;
use crate::bindings::client_event_team_created;

fn test_logging_lib() {
    let team_uuid = CString::new("ff4c8d30-5425-40b2-b5ff-594dac395a14").unwrap();
    let team_name = CString::new("Team Test").unwrap();
    let user_uuid = CString::new("2a65796e-0c66-4dd4-bda1-56eb28392195").unwrap();

    unsafe {
        client_event_team_created(team_uuid.as_ptr(), team_name.as_ptr(), user_uuid.as_ptr());
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    test_logging_lib();

    while let Some(line_result) = lines.next() {
        match line_result {
            Ok(line) => {
                println!("The line : {}", line);
            }
            Err(e) => {
                eprintln!("An error occured reading stdin : {}", e);
            }
        }
    }
}
