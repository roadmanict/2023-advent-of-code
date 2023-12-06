use std::{fs, str::FromStr, vec};

use adventofcode2023::utils::split_whitespace_and_parse_to_usize;

#[test]
fn test_day_6_part_1() {
    let file = fs::read_to_string("resources/day_6.txt").expect("File should be available");

    let mut parsed: Vec<Vec<usize>> = vec![];

    for line in file.lines() {
        let (_, values) = line.split_once(": ").expect("Should be splittable");

        let values = values.trim();

        parsed.push(split_whitespace_and_parse_to_usize(values).expect("Should be ok"));
    }

    println!("{:?}", parsed);

    let times = parsed.get(0).expect("Should be there");
    let distances = parsed.get(1).expect("Should be there");

    let mut result = 1;

    for (i, time) in times.iter().enumerate() {
        let distance = distances[i];

        result = result * calculate(*time, distance);
    }

    assert_eq!(result, 170000);
}

#[test]
fn test_day_6_part_2() {
    let file = fs::read_to_string("resources/day_6.txt").expect("File should be available");

    let mut parsed: Vec<Vec<usize>> = vec![];

    for line in file.lines() {
        let (_, values) = line.split_once(": ").expect("Should be splittable");

        let values = values.replace(' ', "");

        parsed.push(split_whitespace_and_parse_to_usize(&values).expect("Should be ok"));
    }

    println!("{:?}", parsed);

    let times = parsed.get(0).expect("Should be there");
    let distances = parsed.get(1).expect("Should be there");

    let mut result = 1;

    for (i, time) in times.iter().enumerate() {
        let distance = distances[i];

        result = result * calculate(*time, distance);
    }

    assert_eq!(result, 20537782);
}

fn calculate(time: usize, distance: usize) -> usize {
    let mut speed = 0;

    let mut times: usize = 0;

    for speed in 1..time {
        let r = speed * (time - speed);

        if r > distance {
            times += 1;
        }
    }

    times
}
