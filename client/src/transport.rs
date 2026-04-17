pub fn reply_code(reply: &str) -> Option<u16> {
    let code_str = reply.split_whitespace().next()?;
    code_str.parse::<u16>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reply_code_valid() {
        assert_eq!(reply_code("200 OK\r\n"), Some(200));
        assert_eq!(reply_code("404 Not Found"), Some(404));
    }

    #[test]
    fn test_reply_code_invalid() {
        assert_eq!(reply_code("EVENT USER_LOGGED_IN"), None);
        assert_eq!(reply_code(""), None);
        assert_eq!(reply_code("ABC Def"), None);
    }
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
