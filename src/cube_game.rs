use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct CubeGame {
    sets: Vec<CubeGameSet>,
    pub max_red: usize,
    pub max_blue: usize,
    pub max_green: usize,
}

impl CubeGame {
    fn new(sets: Vec<CubeGameSet>) -> Self {
        let max_red = sets
            .iter()
            .map(|s| &s.draws)
            .flatten()
            .filter(|d| d.color == CubeGameCubeColor::Red)
            .map(|d| d.amount)
            .max()
            .unwrap_or(0);

        let max_blue = sets
            .iter()
            .map(|s| &s.draws)
            .flatten()
            .filter(|d| d.color == CubeGameCubeColor::Blue)
            .map(|d| d.amount)
            .max()
            .unwrap_or(0);

        let max_green = sets
            .iter()
            .map(|s| &s.draws)
            .flatten()
            .filter(|d| d.color == CubeGameCubeColor::Green)
            .map(|d| d.amount)
            .max()
            .unwrap_or(0);

        Self {
            sets,
            max_red,
            max_blue,
            max_green,
        }
    }
}

#[derive(Debug, Error)]
pub enum CubeGameFromStrError {
    #[error("Invalid input error")]
    InvalidInputError,
    #[error("Can't parse number error")]
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for CubeGame {
    type Err = CubeGameFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, draws) = s
            .split_once(": ")
            .ok_or(CubeGameFromStrError::InvalidInputError)?;

        let sets = draws
            .split("; ")
            .map(|s| CubeGameSet::from_str(s))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CubeGame::new(sets))
    }
}

#[derive(Debug, PartialEq)]
struct CubeGameSet {
    draws: Vec<CubeGameDraw>,
}

impl CubeGameSet {
    fn new(draws: Vec<CubeGameDraw>) -> Self {
        Self { draws }
    }
}

impl FromStr for CubeGameSet {
    type Err = CubeGameFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let draws = s
            .split(", ")
            .map(|d| CubeGameDraw::from_str(d))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CubeGameSet::new(draws))
    }
}

#[derive(Debug, PartialEq)]
struct CubeGameDraw {
    color: CubeGameCubeColor,
    amount: usize,
}

impl CubeGameDraw {
    fn new(color: CubeGameCubeColor, amount: usize) -> Self {
        Self { color, amount }
    }
}

#[derive(Debug, PartialEq)]
enum CubeGameCubeColor {
    Blue,
    Red,
    Green,
}

impl FromStr for CubeGameDraw {
    type Err = CubeGameFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (amount, cube_color) = s
            .split_once(" ")
            .ok_or(CubeGameFromStrError::InvalidInputError)?;

        let cube_color = match cube_color {
            "blue" => CubeGameCubeColor::Blue,
            "red" => CubeGameCubeColor::Red,
            "green" => CubeGameCubeColor::Green,
            _ => return Err(CubeGameFromStrError::InvalidInputError),
        };

        let amount = amount.parse::<usize>()?;

        Ok(CubeGameDraw::new(cube_color, amount))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_game_from_str() {
        let game = CubeGame::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
            .expect("Should parse input");

        assert_eq!(
            game.sets[0],
            CubeGameSet::new(vec![
                CubeGameDraw::new(CubeGameCubeColor::Blue, 3),
                CubeGameDraw::new(CubeGameCubeColor::Red, 4)
            ])
        );

        assert_eq!(game.max_red, 4);
        assert_eq!(game.max_blue, 6);
        assert_eq!(game.max_green, 2);
    }

    #[test]
    fn test_cube_game_2() {
        let game = CubeGame::from_str("Game 76: 8 green, 6 blue, 5 red; 1 red, 2 blue, 9 green; 7 red, 9 green; 5 green, 1 blue, 11 red")
            .expect("Should parse input");

        assert_eq!(game.max_red, 11);
        assert_eq!(game.max_blue, 6);
        assert_eq!(game.max_green, 9);
    }

    #[test]
    fn test_cube_game() {
        let game = CubeGame::from_str("Game 1: 10 green, 5 blue; 1 red, 9 green, 10 blue; 5 blue, 6 green, 2 red; 7 green, 9 blue, 1 red; 2 red, 10 blue, 10 green; 7 blue, 1 red")
            .expect("Should parse input");

        assert_eq!(
            game.sets[0],
            CubeGameSet::new(vec![
                CubeGameDraw::new(CubeGameCubeColor::Green, 10),
                CubeGameDraw::new(CubeGameCubeColor::Blue, 5),
            ])
        );

        assert_eq!(game.max_red, 2);
        assert_eq!(game.max_blue, 10);
        assert_eq!(game.max_green, 10);
    }

    #[test]
    fn test_cube_game_set_from_str() {
        let set = CubeGameSet::from_str("3 blue, 4 red").expect("Should parse input");

        assert_eq!(
            set,
            CubeGameSet::new(vec![
                CubeGameDraw::new(CubeGameCubeColor::Blue, 3),
                CubeGameDraw::new(CubeGameCubeColor::Red, 4)
            ])
        )
    }

    #[test]
    fn test_cube_game_draw_from_str() {
        let draw = CubeGameDraw::from_str("3 blue").expect("Should parse input");

        assert_eq!(draw, CubeGameDraw::new(CubeGameCubeColor::Blue, 3))
    }
}
