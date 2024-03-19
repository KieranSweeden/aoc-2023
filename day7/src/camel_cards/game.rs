use std::str::FromStr;

use crate::camel_cards::error::Error;
use crate::camel_cards::hand::Hand;

#[derive(Debug)]
pub struct Game {
    pub hands: Vec<Hand>,
}

impl FromStr for Game {
    type Err = Error;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut hands: Vec<Hand> = string
            .lines()
            .map(|line| {
                Hand::from_str(line).expect(format!("Could not parse line {}", line).as_str())
            })
            .collect();

        hands.sort_by(|a, b| b.partial_cmp(&a).unwrap());

        Ok(Self { hands })
    }
}

impl Game {
    pub fn calculate_winnings(&self) -> u64 {
        self.hands.iter().enumerate().fold(0, |acc, (index, hand)| {
            let multiplier = self.hands.len() - index;
            let hand_winnings = multiplier as u64 * hand.bid;
            acc + hand_winnings
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn game_can_be_parsed() -> Result<(), Error> {
        let input = include_str!("../example.txt");
        let game = Game::from_str(input)?;
        assert!(!game.hands.is_empty());
        Ok(())
    }

    #[test]
    fn game_calculates_winnings_correctly() -> Result<(), Error> {
        let input = include_str!("../example.txt");
        let game = Game::from_str(input)?;
        let winnings = game.calculate_winnings();
        assert_eq!(winnings, 6440);
        Ok(())
    }

    #[test]
    fn orders_hands_correctly() -> Result<(), Error> {
        let input = include_str!("../example.txt");
        let game = Game::from_str(input)?;
        let first_hand = game.hands.get(0).unwrap();
        assert_eq!(first_hand.bid, 483);
        let second_hand = game.hands.get(1).unwrap();
        assert_eq!(second_hand.bid, 684);
        let third_hand = game.hands.get(2).unwrap();
        assert_eq!(third_hand.bid, 28);
        let fourth_hand = game.hands.get(3).unwrap();
        assert_eq!(fourth_hand.bid, 220);
        let fifth_hand = game.hands.get(4).unwrap();
        assert_eq!(fifth_hand.bid, 765);
        Ok(())
    }

    #[test]
    fn game_calculates_winnings_correctly_in_joker_mode() -> Result<(), Error> {
        std::env::set_var("JOKER_MODE", "true");
        let input = include_str!("../example.txt");
        let game = Game::from_str(input)?;
        let winnings = game.calculate_winnings();
        assert_eq!(winnings, 5905);
        Ok(())
    }
}
