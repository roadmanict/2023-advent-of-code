use std::fs;

use adventofcode2023::add;

#[test]
fn test_day_1_part_1() {
    let _file = fs::read_to_string("resources/day_1.txt").expect("File should be available");

    assert_eq!(add(1,3), 4);
}

