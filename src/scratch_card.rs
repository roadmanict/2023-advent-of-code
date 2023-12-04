use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

pub struct ScratchCard {
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl ScratchCard {
    fn new(winning_numbers: Vec<usize>, numbers: Vec<usize>) -> Self {
        Self {
            winning_numbers,
            numbers,
        }
    }

    pub fn value(&self) -> usize {
        let mut value = 0;

        for number in self.numbers.iter() {
            if self.winning_numbers.contains(number) {
                if (value == 0) {
                    value = 1;
                } else {
                    value *= 2;
                }
            }
        }

        value
    }

    pub fn wins(&self) -> usize {
        let mut value = 0;

        for number in self.numbers.iter() {
            if self.winning_numbers.contains(number) {
                value += 1;
            }
        }

        value
    }
}

#[derive(Debug, Error)]
pub enum ScratchCardFromStrError {
    #[error("Invalid input error")]
    InvalidInputError,
    #[error("Can't parse number error")]
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for ScratchCard {
    type Err = ScratchCardFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, numbers) = s
            .split_once(": ")
            .ok_or(ScratchCardFromStrError::InvalidInputError)?;
        let (winning_numbers, numbers) = numbers
            .split_once(" | ")
            .ok_or(ScratchCardFromStrError::InvalidInputError)?;
        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(|n| n.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ScratchCard::new(winning_numbers, numbers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scratch_game() {
        let scratch_card =
            ScratchCard::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
                .expect("Should parse input");

        assert_eq!(scratch_card.value(), 8);

        let scratch_card =
            ScratchCard::from_str("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")
                .expect("Should parse input");

        assert_eq!(scratch_card.value(), 2);

        let scratch_card =
            ScratchCard::from_str("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")
                .expect("Should parse input");

        assert_eq!(scratch_card.value(), 2);

        let scratch_card =
            ScratchCard::from_str("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")
                .expect("Should parse input");

        assert_eq!(scratch_card.value(), 0)
    }
}
