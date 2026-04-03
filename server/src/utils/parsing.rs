pub fn parse_quoted_args(input: &str) -> Vec<String> {
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
                    if c == '"' {
                        closed = true;
                        break;
                    }
                    token.push(c);
                }
                if closed != true {
                    continue;
                }
                args.push(token);
            }
            Some(_) => {
                chars.next();
            }
            None => break,
        }
    }

    args
}
