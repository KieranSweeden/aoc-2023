#[derive(Debug, Clone)]
pub struct Card {
    pub id: u32,
    pub winning_numbers: Vec<u32>,
    pub player_numbers: Vec<u32>,
}

impl Card {
    fn split_into_id_and_number_parts(input: &str) -> Result<(&str, &str), &str> {
        let (id_part, numbers_part) = input
            .split_once(':')
            .expect("Failed to split game id part from input");
        Ok((id_part.trim(), numbers_part.trim()))
    }

    fn split_numbers_part(numbers_part: &str) -> Result<(&str, &str), &str> {
        let (winning_numbers, player_numbers) = numbers_part
            .split_once('|')
            .expect("Failed to split winning numbers from player numbers");
        Ok((winning_numbers, player_numbers))
    }

    fn parse_numbers(numbers: &str) -> Result<Vec<u32>, &str> {
        let numbers: Vec<u32> = numbers
            .trim()
            .split_whitespace()
            .map(|number| number.parse().expect("Failed to parse number"))
            .collect();
        Ok(numbers)
    }

    fn parse_numbers_from_part(numbers_part: &str) -> Result<(Vec<u32>, Vec<u32>), &str> {
        let (winning_numbers, player_numbers) = Self::split_numbers_part(numbers_part)?;
        let winning_numbers = Self::parse_numbers(winning_numbers)?;
        let player_numbers = Self::parse_numbers(player_numbers)?;
        Ok((winning_numbers, player_numbers))
    }

    fn parse_id_from_part(id_part: &str) -> Result<u32, &str> {
        let card_id = id_part
            .split_whitespace()
            .nth(1)
            .expect("Failed to split id from card id part");
        let card_id: u32 = card_id.parse().expect("Failed to parse id");
        Ok(card_id)
    }

    pub fn parse(input: &str) -> Result<Self, &str> {
        let (id_part, numbers_part) = Self::split_into_id_and_number_parts(input)?;
        let id = Self::parse_id_from_part(id_part)?;
        let (winning_numbers, player_numbers) = Self::parse_numbers_from_part(numbers_part)?;

        Ok(Self {
            id,
            winning_numbers,
            player_numbers,
        })
    }

    pub fn calculate_points(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|&winning_number| self.player_numbers.contains(&winning_number))
            .fold(0, |acc, _| if acc == 0 { acc + 1 } else { acc * 2 })
    }

    pub fn calculate_intersecting_number_count(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|&winning_number| self.player_numbers.contains(&winning_number))
            .map(|winning_number| winning_number.clone())
            .collect::<Vec<u32>>()
            .len() as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn card_parses_successfully() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratchcard = Card::parse(input);
        match scratchcard {
            Ok(scratchcard) => {
                assert_eq!(scratchcard.id, 1);
                assert_eq!(scratchcard.winning_numbers, vec![41, 48, 83, 86, 17]);
                assert_eq!(
                    scratchcard.player_numbers,
                    vec![83, 86, 6, 31, 17, 9, 48, 53]
                );
            }
            Err(error) => panic!("{error}"),
        }
    }

    #[test]
    fn card_calculates_intersecting_number_count_successfully() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratchcard = Card::parse(input);
        match scratchcard {
            Ok(scratchcard) => {
                assert_eq!(scratchcard.calculate_intersecting_number_count(), 4);
            }
            Err(error) => panic!("{error}"),
        }
    }

    #[test]
    fn card_calculates_points_successfully() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratchcard = Card::parse(input);
        match scratchcard {
            Ok(scratchcard) => {
                let points = scratchcard.calculate_points();
                assert_eq!(points, 8);
            }
            Err(error) => panic!("{error}"),
        }
    }
}
