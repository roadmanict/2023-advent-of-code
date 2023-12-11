use std::fs;

use adventofcode2023::camel_hand::CamelHand;

#[test]
fn test_day_7_part_1() {
    let file = fs::read_to_string("resources/day_7.txt").expect("File should be available");

    for line in file.lines() {

        let _camel_hand: CamelHand = line.parse().expect("Should parse");
    }
}
