use std::{fs, str::FromStr};

use adventofcode2023::scratch_card::ScratchCard;

#[test]
fn test_day_4_part_1() {
    let file = fs::read_to_string("resources/day_4.txt").expect("File should be available");

    assert_eq!(file.lines().count(), 215);

    let mut count_cards: Vec<usize> = vec![1; file.lines().count()];

    let mut total: usize = 0;

    for (i, line) in file.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let times = count_cards[i];
        let card = ScratchCard::from_str(line).expect("Should parse input");
        let wins = card.wins();

        for j in 1..wins + 1 {
            count_cards[i + j] += times;
        }

        total += card.value();
    }

    println!("{:?}", count_cards);

    // part 1
    assert_eq!(total, 27845);

    // part 2
    assert_eq!(count_cards.iter().sum::<usize>(), 9496801);
}
