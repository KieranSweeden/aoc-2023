use std::collections::HashMap;

use super::card::Card;

#[derive(Debug, Clone)]

pub struct Pile {
    pub scratchcards: Vec<Card>,
}

#[allow(dead_code)]
impl Pile {
    fn split_scratchcard_lines(input: &str) -> Vec<&str> {
        input.lines().collect()
    }

    fn parse_scratchcard_lines(scratchcard_lines: Vec<&str>) -> Result<Vec<Card>, &str> {
        scratchcard_lines
            .iter()
            .map(|scratchcard_line| Card::parse(scratchcard_line))
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
                count = count + copy_count;
            }

            // set count for current scratchcard
            scratchcard_count_map.insert(scratchcard.id, count);

            // calculate intersecting number count
            let intersecting_numbers_count = scratchcard.calculate_intersecting_number_count();

            while count > 0 {
                // increment copies for later scratchcards
                for scratchcard_id in
                    (scratchcard.id + 1)..=(scratchcard.id + intersecting_numbers_count)
                {
                    if let Some(copy_count) = scratchcard_count_map.get(&scratchcard_id) {
                        scratchcard_count_map.insert(scratchcard_id, copy_count + 1);
                    } else {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pile_parses_successfully() {
        let input = include!("../sample.txt");
        let scratchcard_pile = Pile::parse(input);
        assert_eq!(
            true,
            scratchcard_pile.is_ok_and(|scratchcard_pile| scratchcard_pile.scratchcards.len() == 6)
        );
    }

    #[test]
    fn pile_calculates_points_successfully() {
        let input = include!("../sample.txt");
        let scratchcard_pile = Pile::parse(input);
        match scratchcard_pile {
            Ok(scratchcard_pile) => {
                assert_eq!(13, scratchcard_pile.calculate_points())
            }
            Err(error) => panic!("{error}"),
        }
    }

    #[test]
    fn pile_calculates_scratchcard_count_successfully() {
        let input = include_str!("../sample.txt");
        let scratchcard_pile = Pile::parse(input);
        match scratchcard_pile {
            Ok(scratchcard_pile) => {
                assert_eq!(scratchcard_pile.calculate_scratchcard_count(), 30);
            }
            Err(error) => panic!("{error}"),
        }
    }
}
