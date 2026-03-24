use crate::bindings::server_event_team_created;
use std::ffi::CString;

mod bindings;

fn main() {
    println!("Hello, world!");

    let team_uuid = CString::new("ff4c8d30-5425-40b2-b5ff-594dac395a14").unwrap();
    let team_name = CString::new("Team Test").unwrap();
    let user_uuid = CString::new("2a65796e-0c66-4dd4-bda1-56eb28392195").unwrap();

    unsafe {
        server_event_team_created(team_uuid.as_ptr(), team_name.as_ptr(), user_uuid.as_ptr());
    }
}
