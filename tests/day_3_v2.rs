use std::fs;

#[derive(Debug)]
struct PartNumber {
    r: usize,
    c: usize,
    len: usize,
    value: usize,
    positions: Vec<(usize, usize)>,
}

impl PartNumber {
    fn new(r: usize, c: usize, len: usize, value: usize) -> Self {
        Self {
            r,
            c,
            len,
            value,
            positions: (c..c + len).map(|i| (r, i)).collect::<Vec<_>>(),
        }
    }

    fn is_overlapping(&self, r: usize, c: usize) -> bool {
        return self.positions.contains(&(r, c));
    }
}

#[test]
fn test_day_3_part_1_v2() {
    let file = fs::read_to_string("resources/day_3.txt").expect("File should be available");

    let mut part_number_positions: Vec<PartNumber> = vec![];
    let mut symbol_position: Vec<(usize, usize)> = vec![];
    let mut star_positions: Vec<(usize, usize)> = vec![];

    for (r, line) in file.lines().enumerate() {
        let mut start_number_pos: Option<(usize, usize)> = None;
        let mut length: usize = 1;
        let mut value: String = String::new();
        for (c, char) in line.chars().enumerate() {
            if char == '*' {
                star_positions.push((r, c));
            }
            if !char.is_digit(10) && char != '.' {
                symbol_position.push((r, c));

                continue;
            } else if char == '.' {
                continue;
            }

            if start_number_pos.is_none() {
                start_number_pos = Some((r, c));
            }

            let next_char_pos = c + 1;
            value.push(char);

            if next_char_pos < line.len()
                && line
                    .chars()
                    .nth(next_char_pos)
                    .expect("Char should exist")
                    .is_digit(10)
            {
                length += 1;

                continue;
            }

            let start_pos = start_number_pos.expect("Should exist");
            let part_number = PartNumber::new(
                start_pos.0,
                start_pos.1,
                length,
                value.parse::<usize>().expect("Value to be parsed"),
            );

            part_number_positions.push(part_number);

            start_number_pos = None;
            length = 1;
            value = String::new();
        }
    }

    let mut total: usize = 0;

    for part_number in part_number_positions.iter() {
        let neighbors = get_neighbors(part_number.r, part_number.c, part_number.len);

        for neighbor in neighbors {
            if symbol_position.contains(&neighbor) {
                total += part_number.value;

                break;
            }
        }
    }

    assert_eq!(total, 554003);

    let mut total: usize = 0;

    for star_position in star_positions {
        let neighbors = get_neighbors(star_position.0, star_position.1, 1);

        let mut parts: Vec<usize> = vec![];

        for neighbor in neighbors {
            for part_position in part_number_positions.iter() {
                if part_position.is_overlapping(neighbor.0, neighbor.1) {
                    if !parts.contains(&part_position.value) {
                        parts.push(part_position.value);
                    }
                }
            }
        }

        if parts.len() > 1 {
            total += parts[0] * parts[1];
        }
    }

    assert_eq!(total, 87263515);
}

fn get_neighbors(r: usize, c: usize, len: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = vec![];
    let r_start = if r > 0 { r - 1 } else { 0 };
    let r_end = r + 1;
    let c_start = if c > 0 { c - 1 } else { 0 };
    let c_end = c + len + 1;

    for cr in r_start..=r_end {
        for cc in c_start..c_end {
            if r == cr && c == cc {
                continue;
            }

            neighbors.push((cr, cc));
        }
    }

    neighbors
}
