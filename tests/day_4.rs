use std::{fs, str::FromStr};

use adventofcode2023::{scratch_card::ScratchCard};

#[test]
fn test_day_4_part_1() {
    let file = fs::read_to_string("resources/day_4.txt").expect("File should be available");

    assert_eq!(file.lines().count(), 215);

    let mut count_cards: Vec<(usize, usize)> = file
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, _)| (i, 1))
        .collect();

    let mut total: usize = 0;

    for (i, line) in file.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let times = count_cards[i].1;
        let card = ScratchCard::from_str(line).expect("Should parse input");
        let wins = card.wins();

        for _ in 0..times {
            for j in 1..wins + 1 {
                count_cards[i + j].1 += 1;
            }
        }

        total += card.value();
    }

    println!("{:?}", count_cards);

    // part 1
    assert_eq!(total, 27845);

    // part 2
    assert_eq!(count_cards.iter().map(|(_, n)| n).sum::<usize>(), 9496801);
}
