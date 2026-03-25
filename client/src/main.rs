use std::io::BufRead;
use libs::ClientLog;

fn test_logging_lib() {
    let _ = ClientLog::client_event_logged_in(
        "2a65796e-0c66-4dd4-bda1-56eb28392195".to_string(),
        "alice".to_string(),
    );
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
