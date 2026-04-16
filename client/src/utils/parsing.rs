pub fn parse_quoted_segments(input: &str) -> Vec<String> {
    input.split('"').skip(1).step_by(2).map(|part| part.to_string()).collect()
}