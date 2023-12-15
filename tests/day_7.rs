use std::fs;

use adventofcode2023::camel_hand::{CamelHand, GameType};

#[test]
fn test_day_7_part_1() {
    let file = fs::read_to_string("resources/day_7.txt").expect("File should be available");

    let mut hands: Vec<CamelHand> = file
        .lines()
        .map(|s| CamelHand::try_from((s, GameType::WithoutJoker)))
        .collect::<Result<Vec<_>, _>>()
        .expect("Should parse");

    hands.sort();

    let result: usize = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum();

    assert_eq!(result, 251106089);
}

#[test]
fn test_day_7_part_2() {
    let file = fs::read_to_string("resources/day_7.txt").expect("File should be available");

    let mut hands: Vec<CamelHand> = file
        .lines()
        .map(|s| CamelHand::try_from((s, GameType::WithJoker)))
        .collect::<Result<Vec<_>, _>>()
        .expect("Should parse");

    hands.sort();

    let result: usize = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum();

    assert_eq!(result, 249620106);
}
