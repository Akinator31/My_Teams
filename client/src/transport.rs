pub fn reply_code(reply: &str) -> Option<u16> {
    let code_str = reply.split_whitespace().next()?;
    code_str.parse::<u16>().ok()
}

pub fn print_colored_reply(reply: &str) {
    let code = reply_code(reply);

    match code {
        Some(c) if c >= 200 && c < 300 => {
            println!("\x1b[32m{}\x1b[0m", reply.trim_end_matches(|c| c == '\r' || c == '\n'));
        }
        Some(c) if c >= 400 && c < 500 => {
            println!("\x1b[31m{}\x1b[0m", reply.trim_end_matches(|c| c == '\r' || c == '\n'));
        }
        Some(c) if c >= 500 => {
            println!("\x1b[31m{}\x1b[0m", reply.trim_end_matches(|c| c == '\r' || c == '\n'));
        }
        _ => {
            println!("{}", reply.trim_end_matches(|c| c == '\r' || c == '\n'));
        }
    }
}
