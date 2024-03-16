use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;
use std::ops::Add;
use std::str::FromStr;

use crate::camel_cards::card::Card;
use crate::camel_cards::error::Error;

#[derive(Debug, PartialEq, Eq, Ord)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use HandType::*;

        Some(match (self, other) {
            (FiveOfAKind, FiveOfAKind) => Ordering::Equal,
            (FiveOfAKind, _) => Ordering::Greater,
            (_, FiveOfAKind) => Ordering::Less,

            (FourOfAKind, FourOfAKind) => Ordering::Equal,
            (FourOfAKind, _) => Ordering::Greater,
            (_, FourOfAKind) => Ordering::Less,

            (FullHouse, FullHouse) => Ordering::Equal,
            (FullHouse, _) => Ordering::Greater,
            (_, FullHouse) => Ordering::Less,

            (ThreeOfAKind, ThreeOfAKind) => Ordering::Equal,
            (ThreeOfAKind, _) => Ordering::Greater,
            (_, ThreeOfAKind) => Ordering::Less,

            (TwoPair, TwoPair) => Ordering::Equal,
            (TwoPair, _) => Ordering::Greater,
            (_, TwoPair) => Ordering::Less,

            (OnePair, OnePair) => Ordering::Equal,
            (OnePair, _) => Ordering::Greater,
            (_, OnePair) => Ordering::Less,

            (HighCard, HighCard) => Ordering::Equal,
        })
    }
}

impl TryFrom<&Vec<Card>> for HandType {
    type Error = Error;

    fn try_from(cards: &Vec<Card>) -> Result<Self, Self::Error> {
        let mut label_counts: HashMap<char, u64> = HashMap::new();
        for card in cards.iter() {
            let label = card.label;
            match label_counts.get(&label) {
                Some(count) => label_counts.insert(label, count.add(1)),
                None => label_counts.insert(label, 1),
            };
        }

        match label_counts.len() {
            1 => Ok(HandType::FiveOfAKind),
            2 => match label_counts.values().nth(0) {
                Some(count) if *count == 1 || *count == 4 => Ok(HandType::FourOfAKind),
                Some(count) if *count == 2 || *count == 3 => Ok(HandType::FullHouse),
                Some(count) => Err(Error::LabelCountError(format!(
                    "Unhandled count {} in case of 2 labels",
                    count
                ))),
                None => Err(Error::LabelCountError(
                    "Count resulted to none in case of 2 labels".to_string(),
                )),
            },
            3 => {
                for count in label_counts.values() {
                    if *count == 3 {
                        return Ok(HandType::ThreeOfAKind);
                    }
                }
                return Ok(HandType::TwoPair);
            }
            4 => Ok(HandType::OnePair),
            5 => Ok(HandType::HighCard),
            _ => Err(Error::LabelCountError(format!(
                "Couldn't catch label count of {}",
                label_counts.len(),
            ))),
        }
    }
}

#[derive(Debug, Ord)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bid: u64,
    pub hand_type: HandType,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match self.hand_type.partial_cmp(&other.hand_type).unwrap() {
            Ordering::Greater => false,
            Ordering::Less => false,
            Ordering::Equal => {
                for (card, other_card) in zip(&self.cards, &other.cards) {
                    match card.partial_cmp(other_card).unwrap() {
                        Ordering::Greater => return false,
                        Ordering::Less => return false,
                        Ordering::Equal => continue,
                    }
                }
                true
            }
        }
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type).unwrap() {
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => {
                for (card, other_card) in zip(&self.cards, &other.cards) {
                    match card.partial_cmp(other_card).unwrap() {
                        Ordering::Greater => return Some(Ordering::Greater),
                        Ordering::Less => return Some(Ordering::Less),
                        Ordering::Equal => continue,
                    }
                }
                Some(Ordering::Equal)
            }
        }
    }
}

impl FromStr for Hand {
    type Err = Error;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (card_labels, bid) = string.split_at(5);

        let cards: Vec<Card> = card_labels.chars().map(|label| Card::new(label)).collect();
        let hand_type = HandType::try_from(&cards)?;

        match bid.trim().parse::<u64>() {
            Ok(bid) => Ok(Self {
                cards,
                bid,
                hand_type,
            }),
            Err(_) => Err(Error::HandNotParsableError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hands_are_ordered_correctly() {
        let greater_hand = Hand::from_str("QTJ4Q 319").unwrap();
        let lesser_hand = Hand::from_str("QT5QA 749").unwrap();
        assert_eq!(
            greater_hand.partial_cmp(&lesser_hand),
            Some(Ordering::Greater)
        );

        let greater_hand = Hand::from_str("QTJAQ 148").unwrap();
        let lesser_hand = Hand::from_str("QTJ4Q 319").unwrap();
        assert_eq!(
            greater_hand.partial_cmp(&lesser_hand),
            Some(Ordering::Greater)
        );
    }

    #[test]
    fn hand_can_be_parsed_from_str() -> Result<(), Error> {
        let input = "32T3K 765";
        let hand = Hand::from_str(input)?;
        assert!(!hand.cards.is_empty());
        Ok(())
    }

    #[test]
    fn can_detect_five_of_a_kind_when_parsing_hand_from_str() -> Result<(), Error> {
        let input = "AAAAA 765";
        let hand = Hand::from_str(input)?;
        assert!(matches!(hand.hand_type, HandType::FiveOfAKind));
        Ok(())
    }

    #[test]
    fn can_detect_four_of_a_kind_when_parsing_hand_from_str() -> Result<(), Error> {
        let input = "AAAAJ 765";
        let hand = Hand::from_str(input)?;
        assert!(matches!(hand.hand_type, HandType::FourOfAKind));
        Ok(())
    }

    #[test]
    fn can_detect_full_house_when_parsing_hand_from_str() -> Result<(), Error> {
        let input = "23322 765";
        let hand = Hand::from_str(input)?;
        assert!(matches!(hand.hand_type, HandType::FullHouse));
        Ok(())
    }

    #[test]
    fn can_detect_three_of_a_kind_when_parsing_hand_from_str() -> Result<(), Error> {
        let input = "TTT98 765";
        let hand = Hand::from_str(input)?;
        assert!(matches!(hand.hand_type, HandType::ThreeOfAKind));
        Ok(())
    }

    #[test]
    fn can_detect_two_pair_when_parsing_hand_from_str() -> Result<(), Error> {
        let input = "23432 765";
        let hand = Hand::from_str(input)?;
        assert!(matches!(hand.hand_type, HandType::TwoPair));
        Ok(())
    }

    #[test]
    fn can_detect_one_pair_when_parsing_hand_from_str() -> Result<(), Error> {
        let input = "A23A4 765";
        let hand = Hand::from_str(input)?;
        assert!(matches!(hand.hand_type, HandType::OnePair));
        Ok(())
    }

    #[test]
    fn can_detect_high_card_when_parsing_hand_from_str() -> Result<(), Error> {
        let input = "45678 765";
        let hand = Hand::from_str(input)?;
        assert!(matches!(hand.hand_type, HandType::HighCard));
        Ok(())
    }
}
