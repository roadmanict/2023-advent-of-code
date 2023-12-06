use std::{f32::INFINITY, num::ParseIntError, str::FromStr};

use thiserror::Error;

use crate::utils::split_whitespace_and_parse_to_usize;

#[derive(Debug)]
pub struct FarmMap {
    lines: Vec<FarmMapLine>,
}

impl FarmMap {
    pub fn new(lines: Vec<FarmMapLine>) -> Self {
        Self { lines }
    }

    pub fn correspond(&self, value: usize) -> Result<usize, FarmMapParseError> {
        let mut result: Option<usize> = None;

        let mut lines_iter = self.lines.iter();

        let mut next_line = lines_iter.next();

        loop {
            if let Some(line) = next_line {
                if value >= line.source_range + line.range_length {
                    next_line = lines_iter.next();

                    continue;
                }

                if value < line.source_range {
                    return Ok(value);
                }

                return Ok(value + line.destination_range - line.source_range);
            } else {
                return Ok(value);
            }
        }
    }

    pub fn correspond_range(
        &self,
        range: &(usize, usize),
    ) -> Result<Vec<(usize, usize)>, FarmMapParseError> {
        let mut result: Option<usize> = None;

        let mut lines_iter = self.lines.iter();

        let mut next_line = lines_iter.next();

        let mut start = range.0;
        let end = range.1;

        let mut ranges: Vec<(usize, usize)> = vec![];

        for line in self.lines.iter() {
            println!("range: {}, {}; line: {:?}", start, end, line);
            if end < line.source_range {
                println!("Ends before line");

                continue;
            }
            let line_end = line.source_range + line.range_length;
            if start >= line_end {
                println!("Starts after line");

                continue;
            }

            if start < line.source_range {
                println!("Starts before line");
                ranges.push((start, line.source_range));
                start = line.source_range;
            }

            let difference = start + line.destination_range - line.source_range;
            println!(
                "difference: {}, {}, {}",
                difference, line.destination_range, line.source_range
            );

            if (line_end < end) {
                println!("Line ends before end");
                ranges.push((start + difference, line_end + difference));
                start = line_end;
            } else {
                println!("Ends after line");
                ranges.push((line_end, end));
            }

            println!("result: {:?}", ranges);
        }

        if (ranges.len() == 0) {
            return Ok(vec![*range]);
        }

        Ok(ranges)
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
pub struct FarmMapLine {
    destination_range: usize,
    source_range: usize,
    range_length: usize,
}

impl FarmMapLine {
    pub fn new(destination_range: usize, source_range: usize, range_lengh: usize) -> Self {
        Self {
            destination_range,
            source_range,
            range_length: range_lengh,
        }
    }
}

impl PartialOrd for FarmMapLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.source_range.partial_cmp(&other.source_range)
    }
}

#[derive(Debug, Error)]
pub enum FarmMapParseError {
    #[error("Invalid input error")]
    InvalidInputError,
    #[error("Can't parse number error")]
    ParseIntError(#[from] ParseIntError),
}

impl TryFrom<Vec<&str>> for FarmMap {
    type Error = FarmMapParseError;

    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        let mut lines: Vec<FarmMapLine> = vec![];
        for s in value.iter() {
            let ranges = split_whitespace_and_parse_to_usize(s)?;

            lines.push(s.parse()?);
        }

        lines.sort();

        Ok(FarmMap::new(lines))
    }
}

impl FromStr for FarmMapLine {
    type Err = FarmMapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = split_whitespace_and_parse_to_usize(s)?;

        Ok(FarmMapLine::new(ranges[0], ranges[1], ranges[2]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_farm_map_try_from() {
        let farm_map: FarmMap = vec!["50 98 2", "52 50 48"]
            .try_into()
            .expect("Input to be parsed");

        assert_eq!(farm_map.lines.len(), 2);
        assert_eq!(
            farm_map.lines[0],
            "52 50 48".parse().expect("input to be parsed")
        );
        assert_eq!(
            farm_map.lines[1],
            "50 98 2".parse().expect("input to be parsed")
        );

        assert_eq!(farm_map.correspond(13).expect("Should return value"), 13);
        assert_eq!(farm_map.correspond(14).expect("Should return value"), 14);
        assert_eq!(farm_map.correspond(55).expect("Should return value"), 57);
        assert_eq!(farm_map.correspond(79).expect("Should return value"), 81);
    }

    #[test]
    fn test_farm_map_try_from_2() {
        let farm_map: FarmMap = vec!["0 15 37", "37 52 2", "39 0 15"]
            .try_into()
            .expect("Input to be parsed");

        assert_eq!(farm_map.lines.len(), 3);
        assert_eq!(farm_map.correspond(81).expect("Should return value"), 81);
        assert_eq!(farm_map.correspond(14).expect("Should return value"), 53);
        assert_eq!(farm_map.correspond(57).expect("Should return value"), 57);
        assert_eq!(farm_map.correspond(13).expect("Should return value"), 52);
    }

    #[test]
    fn test_farm_map_try_from_3() {
        let farm_map: FarmMap = vec!["49 53 8", "0 11 42", "42 0 7", "57 7 4"]
            .try_into()
            .expect("Input to be parsed");

        assert_eq!(farm_map.lines.len(), 4);
        assert_eq!(farm_map.correspond(81).expect("Should return value"), 81);
        assert_eq!(farm_map.correspond(53).expect("Should return value"), 49);
        assert_eq!(farm_map.correspond(57).expect("Should return value"), 53);
        assert_eq!(farm_map.correspond(52).expect("Should return value"), 41);
    }

    #[test]
    fn test_farm_map_try_from_4() {
        let farm_map: FarmMap = vec!["49 53 8", "0 11 42", "42 0 7", "57 7 4"]
            .try_into()
            .expect("Input to be parsed");

        assert_eq!(farm_map.lines.len(), 4);
        assert_eq!(farm_map.correspond_range(&(0, 50)).expect("Should return value"), vec![(81, 88)]);
    }
}
