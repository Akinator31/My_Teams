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

pub fn remove_quoted(input: &str) -> &str {
    if input.starts_with('"') && input.ends_with('"') && input.len() >= 2 {
        &input[1..input.len() - 1]
    } else {
        input
    }
}

pub fn split_args(line: &str) -> Vec<&str> {
    let mut args = Vec::new();
    let mut start = 0;
    let mut depth = 0;
    let mut in_quotes = false;

    for (i, c) in line.char_indices() {
        match c {
            '"' if depth == 0 => in_quotes = !in_quotes,
            '[' if !in_quotes => depth += 1,
            ']' if !in_quotes => depth -= 1,
            ' ' if !in_quotes && depth == 0 => {
                if start < i {
                    args.push(&line[start..i]);
                }
                start = i + 1;
            }
            _ => {}
        }
    }

    if start < line.len() {
        args.push(&line[start..]);
    }

    args
}
