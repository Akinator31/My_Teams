pub fn parse_quoted_segments(input: &str) -> Vec<String> {
    input.split('"').skip(1).step_by(2).map(|part| part.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_quoted_segments() {
        let input = "\"Part 1\" \"Part 2\" \"Part 3\"";
        let segments = parse_quoted_segments(input);
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0], "Part 1");
        assert_eq!(segments[1], "Part 2");
        assert_eq!(segments[2], "Part 3");
    }

    #[test]
    fn test_parse_quoted_segments_empty() {
        let input = "No quotes here";
        let segments = parse_quoted_segments(input);
        assert_eq!(segments.len(), 0);
    }
}