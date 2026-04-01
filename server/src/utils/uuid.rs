use std::fs::File;
use std::io::Read;

pub fn get_uuid() -> String {
    #[cfg(target_os = "linux")]
    {
        let mut uuid = String::new();
        let mut uuid_file = File::open("/proc/sys/kernel/random/uuid").unwrap();
        uuid_file.read_to_string(&mut uuid).expect("/proc/sys/kernel/random/uuid doesn't exist!");
        uuid.trim().to_string()
    }

    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("uuidgen")
            .output()
            .expect("failed to run uuidgen");
        String::from_utf8(output.stdout)
            .expect("invalid utf8")
            .trim()
            .to_lowercase()
    }
}