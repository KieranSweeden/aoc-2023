use std::collections::HashMap;

#[derive(Debug, Clone)]

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

    pub fn calculate_scratchcard_count(&self) -> u32 {
        // track count against each scratchcard via a map against id
        let mut scratchcard_count_map: HashMap<u32, u32> = HashMap::new();

        // iterate over each scratchcard
        for scratchcard in &self.scratchcards {
            // create count starting from 1
            let mut count = 1;

            // get saved count for id if one exists
            if let Some(copy_count) = scratchcard_count_map.get(&scratchcard.id) {
                // add saved count to count
                // println!("Copy count found for id: {}", scratchcard.id);
                count = count + copy_count;
            } else {
                // println!("No copy count found for id: {}", scratchcard.id);
            }

            // println!("Count for id {} is {}", scratchcard.id, count);

            // set count for current scratchcard
            scratchcard_count_map.insert(scratchcard.id, count);

            // calculate intersecting number count
            let intersecting_numbers_count = scratchcard.calculate_intersecting_number_count();
            // println!(
            //     "Intersecting count for id {} is {}",
            //     scratchcard.id, intersecting_numbers_count
            // );

            // println!(
            //     "Iterating from {} to {}",
            //     scratchcard.id + 1,
            //     intersecting_numbers_count
            // );

            while count > 0 {
                // println!("Doing work as count is {count}");

                // increment copies for later scratchcards
                for scratchcard_id in
                    (scratchcard.id + 1)..=(scratchcard.id + intersecting_numbers_count)
                {
                    if let Some(copy_count) = scratchcard_count_map.get(&scratchcard_id) {
                        // println!(
                        //     "Incrementing copy count by 1 for id {} evaluating to {}",
                        //     scratchcard_id,
                        //     copy_count + 1
                        // );
                        scratchcard_count_map.insert(scratchcard_id, copy_count + 1);
                    } else {
                        // println!("Setting copy count for id {} to 1", { scratchcard_id });
                        scratchcard_count_map.insert(scratchcard_id, 1);
                    }
                }

                count = count - 1;
            }
        }

        dbg!(&scratchcard_count_map);

        scratchcard_count_map
            .values()
            .fold(0, |acc, count| acc + count)
    }
}

#[derive(Debug, Clone)]
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

    pub fn calculate_intersecting_number_count(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|&winning_number| self.player_numbers.contains(&winning_number))
            .map(|winning_number| winning_number.clone())
            .collect::<Vec<u32>>()
            .len() as u32
    }
}

fn process(input: &str) -> Result<u32, &str> {
    let scratchcard_pile = ScratchCardPile::parse(input)?;
    Ok(scratchcard_pile.calculate_scratchcard_count())
}

fn main() {
    let input = include_str!("input.txt");
    let result = process(input);
    match result {
        Ok(result) => println!("The result is {result}"),
        Err(error) => panic!("{error}"),
    }
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
    fn scratchcard_pile_calculates_scratchcard_count_successfully() {
        let input = include_str!("sample.txt");
        let scratchcard_pile = ScratchCardPile::parse(input);
        match scratchcard_pile {
            Ok(scratchcard_pile) => {
                assert_eq!(scratchcard_pile.calculate_scratchcard_count(), 30);
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
    fn scratchcard_calculates_intersecting_number_count_successfully() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratchcard = ScratchCard::parse(input);
        match scratchcard {
            Ok(scratchcard) => {
                assert_eq!(scratchcard.calculate_intersecting_number_count(), 4);
            }
            Err(error) => panic!("{error}"),
        }
    }
}
