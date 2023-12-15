use std::{
    collections::HashMap,
    num::{ParseIntError, TryFromIntError},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CamelHandParseError {
    #[error("Invalid input error")]
    InvalidInputError,
    #[error("Can't parse number error")]
    ParseIntError(#[from] ParseIntError),
    #[error("Can't parse number error")]
    TryFromIntError(#[from] TryFromIntError),
}

#[derive(Debug)]
pub enum GameType {
    WithJoker,
    WithoutJoker,
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub enum CamelCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<(char, &GameType)> for CamelCard {
    type Error = CamelHandParseError;

    fn try_from(value: (char, &GameType)) -> Result<Self, Self::Error> {
        match value {
            ('A', _) => Ok(CamelCard::Ace),
            ('K', _) => Ok(CamelCard::King),
            ('Q', _) => Ok(CamelCard::Queen),
            ('J', GameType::WithoutJoker) => Ok(CamelCard::Jack),
            ('J', GameType::WithJoker) => Ok(CamelCard::Joker),
            ('T', _) => Ok(CamelCard::Ten),
            ('9', _) => Ok(CamelCard::Nine),
            ('8', _) => Ok(CamelCard::Eight),
            ('7', _) => Ok(CamelCard::Seven),
            ('6', _) => Ok(CamelCard::Six),
            ('5', _) => Ok(CamelCard::Five),
            ('4', _) => Ok(CamelCard::Four),
            ('3', _) => Ok(CamelCard::Three),
            ('2', _) => Ok(CamelCard::Two),
            _ => Err(CamelHandParseError::InvalidInputError),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum CamelHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Ord, Eq)]
pub struct CamelHand {
    cards: Vec<CamelCard>,
    pub bid: usize,
    pub strength: CamelHandType,
}

impl TryFrom<(&str, GameType)> for CamelHand {
    type Error = CamelHandParseError;

    fn try_from((value, game_type): (&str, GameType)) -> Result<Self, Self::Error> {
        let (hand, bid) = value
            .split_once(' ')
            .ok_or(CamelHandParseError::InvalidInputError)?;

        let cards: Vec<CamelCard> = hand
            .chars()
            .map(|c| (c, &game_type))
            .map(CamelCard::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let mut card_count: HashMap<&CamelCard, (usize, &CamelCard)> = HashMap::new();
        let joker_count = cards.iter().filter(|c| c == &&CamelCard::Joker).count();

        for card in cards.iter() {
            if card_count.contains_key(card.to_owned()) {
                continue;
            }
            let count = cards.iter().filter(|c| c == &card).count();

            card_count.insert(card, (count, &card));
        }

        let mut counted_cards: Vec<(usize, &CamelCard)> =
            card_count.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
        counted_cards.sort_by(|a, b| b.0.cmp(&a.0));

        if joker_count > 0 && joker_count < 5 {
            let mut joker_index: Option<usize> = None;
            for (i, counted_card) in counted_cards.iter_mut().enumerate() {
                if counted_card.1 == &CamelCard::Joker {
                    joker_index = Some(i);

                    continue;
                }

                counted_card.0 += joker_count;

                break;
            }

            if let Some(i) = joker_index {
                counted_cards.remove(i);

            }
        }

        let strength = match counted_cards.as_slice() {
            [(5, _), ..] => Ok(CamelHandType::FiveOfAKind),
            [(4, _), ..] => Ok(CamelHandType::FourOfAKind),
            [(3, _), (2, _), ..] => Ok(CamelHandType::FullHouse),
            [(3, _), ..] => Ok(CamelHandType::ThreeOfAKind),
            [(2, _), (2, _), ..] => Ok(CamelHandType::TwoPair),
            [(2, _), ..] => Ok(CamelHandType::OnePair),
            [(1, _), ..] => Ok(CamelHandType::HighCard),
            _ => Err(CamelHandParseError::InvalidInputError),
        }?;

        Ok(CamelHand {
            cards,
            bid: bid.parse()?,
            strength,
        })
    }
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let result = self.strength.partial_cmp(&other.strength)?;

        if result == std::cmp::Ordering::Equal {
            return self.cards.partial_cmp(&other.cards);
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_card() {
        let joker: CamelCard = ('J', &GameType::WithJoker).try_into().expect("Should work");
        assert_eq!(joker, CamelCard::Joker);
        let jack: CamelCard = ('J', &GameType::WithoutJoker)
            .try_into()
            .expect("Should work");
        assert_eq!(jack, CamelCard::Jack);
        let two: CamelCard = ('2', &GameType::WithoutJoker)
            .try_into()
            .expect("Should work");
        assert_eq!(two, CamelCard::Two);

        assert_eq!(joker.partial_cmp(&jack), Some(core::cmp::Ordering::Less));
        assert_eq!(joker.partial_cmp(&two), Some(core::cmp::Ordering::Less));
    }

    #[test]
    fn test_camel_card_one_pair() {
        let hand: CamelHand = ("32T3K 765", GameType::WithoutJoker)
            .try_into()
            .expect("Should parse");

        assert_eq!(hand.strength, CamelHandType::OnePair);
    }

    #[test]
    fn test_camel_card_three_of_a_kind() {
        let hand: CamelHand = ("T55J5 684", GameType::WithoutJoker)
            .try_into()
            .expect("Should parse");

        assert_eq!(hand.strength, CamelHandType::ThreeOfAKind);
    }

    #[test]
    fn test_camel_card_sort() {
        let mut hands: Vec<CamelHand> = vec![
            ("32T3K 765", GameType::WithoutJoker)
                .try_into()
                .expect("Should parse"),
            ("T55J5 684", GameType::WithoutJoker)
                .try_into()
                .expect("Should parse"),
            ("A55J5 684", GameType::WithoutJoker)
                .try_into()
                .expect("Should parse"),
            ("KK677 28", GameType::WithoutJoker)
                .try_into()
                .expect("Should parse"),
        ];

        hands.sort();

        assert_eq!(hands[0].strength, CamelHandType::OnePair);
        assert_eq!(hands[1].strength, CamelHandType::TwoPair);
        assert_eq!(hands[2].strength, CamelHandType::ThreeOfAKind);
        assert_eq!(hands[3].cards[0], CamelCard::Ace);
        assert_eq!(hands[3].strength, CamelHandType::ThreeOfAKind);
    }

    #[test]
    fn test_camel_card_jokerr() {
        let hand: CamelHand = ("JKKJA 765", GameType::WithJoker)
            .try_into()
            .expect("Should parse");

        assert_eq!(hand.strength, CamelHandType::FourOfAKind);
    }
}
