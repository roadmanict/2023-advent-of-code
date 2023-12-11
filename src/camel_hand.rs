use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

pub struct CamelHand {}

#[derive(Debug, Error)]
pub enum CamelHandParseError {
    #[error("Invalid input error")]
    InvalidInputError,
    #[error("Can't parse number error")]
    ParseIntError(#[from] ParseIntError),
}

pub enum CamelHandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

pub struct CamelCard(char);

impl CamelCard {
    pub fn value(&self) -> usize {
        todo!()
    }
}

impl From<char> for CamelCard {
    fn from(value: char) -> Self {
        CamelCard(value)
    }
}

impl FromStr for CamelHand {
    type Err = CamelHandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s
            .split_once(' ')
            .ok_or(CamelHandParseError::InvalidInputError)?;

        let cards: Vec<CamelCard> = hand.chars().map(|c| c.into()).collect();
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_card_one_pair() {
        let hand: CamelHand = "32T3K 765".parse().expect("Should parse");
    }
}
