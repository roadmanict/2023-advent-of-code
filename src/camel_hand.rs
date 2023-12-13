use std::{
    collections::HashMap,
    num::{ParseIntError, TryFromIntError},
    str::FromStr,
};

use thiserror::Error;

#[derive(Debug, PartialEq, Ord, Eq)]
pub struct CamelHand {
    cards: Vec<CamelCard>,
    first_card: CamelCard,
    pub bid: usize,
    strength: CamelHandType,
}

impl CamelHand {
    pub fn new(
        cards: Vec<CamelCard>,
        first_card: CamelCard,
        bid: usize,
        strength: CamelHandType,
    ) -> Self {
        Self {
            cards,
            first_card,
            bid,
            strength,
        }
    }
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let result = self.strength.partial_cmp(&other.strength)?;

        if result == std::cmp::Ordering::Equal {
            return other.cards.partial_cmp(&self.cards)
        }

        Some(result)
    }
}

#[derive(Debug, Error)]
pub enum CamelHandParseError {
    #[error("Invalid input error")]
    InvalidInputError,
    #[error("Can't parse number error")]
    ParseIntError(#[from] ParseIntError),
    #[error("Can't parse number error")]
    TryFromIntError(#[from] TryFromIntError),
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum CamelHandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
pub struct CamelCard(char, usize);

impl PartialOrd for CamelCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl TryFrom<char> for CamelCard {
    type Error = CamelHandParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let usize_value: usize = match value {
            x if x.is_digit(10) => {
                let raw_value = x
                    .to_digit(10)
                    .ok_or(CamelHandParseError::InvalidInputError)?;

                Ok(raw_value.try_into()?)
            }
            'T' => Ok(10),
            'J' => Ok(11),
            'Q' => Ok(12),
            'K' => Ok(13),
            'A' => Ok(14),
            _ => Err(CamelHandParseError::InvalidInputError),
        }?;

        Ok(CamelCard(value, usize_value))
    }
}

impl FromStr for CamelHand {
    type Err = CamelHandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s
            .split_once(' ')
            .ok_or(CamelHandParseError::InvalidInputError)?;

        let cards: Vec<CamelCard> = hand
            .chars()
            .map(|c| CamelCard::try_from(c))
            .collect::<Result<Vec<_>, _>>()?;
        let first_card = cards
            .get(0)
            .ok_or(CamelHandParseError::InvalidInputError)?
            .to_owned();

        let mut card_count = HashMap::new();

        for card in cards.iter() {
            if card_count.contains_key(&card.0) {
                continue;
            }
            let count = cards.iter().filter(|c| c.0 == card.0).count();

            card_count.insert(card.0, (count, card));
        }

        let mut counted_cards: Vec<&(usize, &CamelCard)> = card_count.values().collect::<Vec<_>>();
        counted_cards.sort_by(|a, b| b.0.cmp(&a.0));

        println!("{:?}", counted_cards);

        let strength = match counted_cards.as_slice() {
            [(5, _), ..] => Ok(CamelHandType::FiveOfAKind),
            [(4, _), ..] => Ok(CamelHandType::FourOfAKind),
            [(3, _), (2, _), ..] => Ok(CamelHandType::FullHouse),
            [(3, _), ..] => Ok(CamelHandType::ThreeOfAKind),
            [(2, _), (2, _), ..] => Ok(CamelHandType::TwoPair),
            [(2, _), ..] => Ok(CamelHandType::OnePair),
            [(1, _), ..] => Ok(CamelHandType::HighCard),
            _ => Err(CamelHandParseError::InvalidInputError),
        };

        println!("{:?}", strength);

        Ok(CamelHand::new(cards, first_card, bid.parse()?, strength?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_card_one_pair() {
        let hand: CamelHand = "32T3K 765".parse().expect("Should parse");

        assert_eq!(hand.strength, CamelHandType::OnePair);
    }

    #[test]
    fn test_camel_card_three_of_a_kind() {
        let hand: CamelHand = "T55J5 684".parse().expect("Should parse");

        assert_eq!(hand.strength, CamelHandType::ThreeOfAKind);
    }

    #[test]
    fn test_camel_card_sort() {
        let mut hands: Vec<CamelHand> = vec![
            "32T3K 765".parse().expect("Should parse"),
            "T55J5 684".parse().expect("Should parse"),
            "A55J5 684".parse().expect("Should parse"),
            "KK677 28".parse().expect("Should parse"),
        ];

        hands.sort();

        assert_eq!(hands[0].strength, CamelHandType::ThreeOfAKind);
        assert_eq!(hands[0].cards[0].0, 'A');
        assert_eq!(hands[1].strength, CamelHandType::ThreeOfAKind);
        assert_eq!(hands[2].strength, CamelHandType::TwoPair);
        assert_eq!(hands[3].strength, CamelHandType::OnePair);
    }
}
