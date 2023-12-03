use std::{fs, str::FromStr};

use adventofcode2023::cube_game::CubeGame;

#[test]
fn test_day_1_part_1() {
    let file = fs::read_to_string("resources/day_2.txt").expect("File should be available");

    let mut games: Vec<CubeGame> = vec![];

    for (_i, line) in file.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let game = CubeGame::from_str(line).expect("Should parse input");

        games.push(game);
    }
    assert_eq!(games.len(), 100);

    let game = &games[75];
    println!(
        "red: {}, blue: {}, green: {}",
        game.max_red, game.max_blue, game.max_green
    );

    let total_indexes_of_valid_games = games
        .iter()
        .enumerate()
        .filter(|(_i, g)| {
            let invalid = g.max_red > 12 || g.max_blue > 14 || g.max_green > 13;

            !invalid
        })
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    assert_eq!(total_indexes_of_valid_games, 2006);
}
