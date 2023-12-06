use std::num::ParseIntError;

pub fn split_whitespace_and_parse_to_usize(s: &str) -> Result<Vec<usize>, ParseIntError> {
    let result = s.split_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>();

    result
}
