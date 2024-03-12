use std::str::FromStr;

use crate::boat_races::race::Race;

#[derive(Debug)]
pub struct ParseBoatRacesFromStringError;

#[derive(Debug)]
pub struct SingleRaceBoatRaceEvent {
    pub race: Race,
}

impl FromStr for SingleRaceBoatRaceEvent {
    type Err = ParseBoatRacesFromStringError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = string.lines().collect();
        let time = lines
            .get(0)
            .expect("Failed to get line repesenting time")
            .split_once(':')
            .expect("Failed to split line")
            .1
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .expect("Failed to parse time into a number");

        let record_distance = lines
            .get(1)
            .expect("Failed to get line repesenting record distance")
            .split_once(':')
            .expect("Failed to split line")
            .1
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .expect("Failed to parse record distance into a number");

        let race = Race::new(time, record_distance);

        Ok(Self { race })
    }
}

#[derive(Debug)]
pub struct BoatRaceEvent {
    races: Vec<Race>,
}

impl BoatRaceEvent {
    pub fn multiply_number_of_ways_each_race_record_can_be_broken(&self) -> u64 {
        self.races.iter().fold(1, |acc, race| {
            acc * race.get_number_of_ways_record_distance_can_be_broken()
        })
    }
}

impl FromStr for BoatRaceEvent {
    type Err = ParseBoatRacesFromStringError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut time_list: Vec<u64> = vec![];
        let mut record_distance_list: Vec<u64> = vec![];

        for (line_index, line) in string.lines().enumerate() {
            match line.split_once(':') {
                Some((_, races)) => {
                    races.split_whitespace().for_each(|num| {
                        let num = num.parse::<u64>().expect("Failed to parse number");
                        if line_index == 0 {
                            time_list.push(num);
                        } else {
                            record_distance_list.push(num);
                        }
                    });
                }
                None => return Err(ParseBoatRacesFromStringError),
            }
        }

        let mut races: Vec<Race> = vec![];
        for (index, time) in time_list.iter().enumerate() {
            let record_distance = record_distance_list
                .get(index)
                .expect("Failed to get distance");
            races.push(Race::new(time.clone(), record_distance.clone()))
        }

        Ok(Self { races })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing_boat_races_from_string_succeeds() -> Result<(), ParseBoatRacesFromStringError> {
        let input = include_str!("../../example.txt");
        let boat_races = BoatRaceEvent::from_str(input)?;
        let first_race = boat_races
            .races
            .get(0)
            .expect("Failed to retrieve first race");
        assert_eq!(first_race.time, 7);
        assert_eq!(first_race.record_distance, 9);
        let second_race = boat_races
            .races
            .get(1)
            .expect("Failed to retrieve second race");
        assert_eq!(second_race.time, 15);
        assert_eq!(second_race.record_distance, 40);
        let third_race = boat_races
            .races
            .get(2)
            .expect("Failed to retrieve third race");
        assert_eq!(third_race.time, 30);
        assert_eq!(third_race.record_distance, 200);
        Ok(())
    }
}
