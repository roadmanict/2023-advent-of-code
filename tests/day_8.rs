use std::fs;

use adventofcode2023::network::Network;
#[test]
fn test_day_8_part_1() {
    let file = fs::read_to_string("resources/day_8.txt").expect("File should be available");

    let network: Network = file.try_into().expect("Should parse");

    assert_eq!(network.walk().expect("Should find answer"), 0);
}
