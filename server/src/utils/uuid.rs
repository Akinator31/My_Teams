use std::fs::File;
use std::io::Read;

fn get_uuid() -> String {
    let mut uuid = String::new();
    let mut uuid_file = File::open("").unwrap();

    uuid_file.read_to_string(&mut uuid).expect("/proc/sys/kernel/random/uuid doesn't exist!");
    uuid
}