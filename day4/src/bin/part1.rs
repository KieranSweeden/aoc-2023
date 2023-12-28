#[derive(Debug)]
struct ScratchCardPile {
    scratchcards: Vec<ScratchCard>,
}

impl ScratchCardPile {
    fn split_scratchcard_lines(input: &str) -> Vec<&str> {
        input.lines().collect()
    }

    fn parse_scratchcard_lines(scratchcard_lines: Vec<&str>) -> Result<Vec<ScratchCard>, &str> {
        scratchcard_lines
            .iter()
            .map(|scratchcard_line| ScratchCard::parse(scratchcard_line))
            .collect()
    }

    pub fn parse(input: &str) -> Result<Self, &str> {
        let scratchcard_lines = Self::split_scratchcard_lines(input);
        let scratchcards = Self::parse_scratchcard_lines(scratchcard_lines)?;

        Ok(Self { scratchcards })
    }

    pub fn calculate_points(&self) -> u32 {
        self.scratchcards
            .iter()
            .fold(0, |acc, scratchcard| acc + scratchcard.calculate_points())
    }
}

#[derive(Debug)]
struct ScratchCard {
    id: u32,
    winning_numbers: Vec<u32>,
    player_numbers: Vec<u32>,
}

impl ScratchCard {
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
}

fn process(input: &str) -> Result<u32, &str> {
    todo!()
}

fn main() {
    println!("Hello, part 1!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scratchcard_pile_parses_successfully() {
        let input = include_str!("sample.txt");
        let scratchcard_pile = ScratchCardPile::parse(input);
        assert_eq!(
            true,
            scratchcard_pile.is_ok_and(|scratchcard_pile| scratchcard_pile.scratchcards.len() == 6)
        );
    }

    #[test]
    fn scratchcard_pile_calculates_points_successfully() {
        let input = include_str!("sample.txt");
        let scratchcard_pile = ScratchCardPile::parse(input);
        match scratchcard_pile {
            Ok(scratchcard_pile) => {
                assert_eq!(13, scratchcard_pile.calculate_points())
            }
            Err(error) => panic!("{error}"),
        }
    }

    #[test]
    fn scratchcard_parses_successfully() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratchcard = ScratchCard::parse(input);
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
    fn scratchcard_calculates_points_successfully() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratchcard = ScratchCard::parse(input);
        match scratchcard {
            Ok(scratchcard) => {
                let points = scratchcard.calculate_points();
                assert_eq!(points, 8);
            }
            Err(error) => panic!("{error}"),
        }
    }
}
