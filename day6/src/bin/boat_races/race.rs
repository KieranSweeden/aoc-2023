#[derive(Debug)]
pub struct Race {
    pub time: u32,
    pub record_distance: u32,
}

impl Race {
    pub fn new(time: u32, record_distance: u32) -> Self {
        Race {
            time,
            record_distance,
        }
    }

    pub fn get_number_of_ways_record_distance_can_be_broken(&self) -> u32 {
        let mut number_of_ways_record_distance_can_be_broken = 0;
        for num in 0..self.time {
            let time_spent_holding_button_down = num;
            let time_remaining = self.time - time_spent_holding_button_down;
            let boat_speed = time_spent_holding_button_down;
            let distance_travelled = time_remaining * boat_speed;
            if distance_travelled > self.record_distance {
                number_of_ways_record_distance_can_be_broken += 1;
            }
        }

        number_of_ways_record_distance_can_be_broken
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    use crate::boat_races::event::{BoatRaceEvent, ParseBoatRacesFromStringError};
    use std::str::FromStr;

    #[test]
    fn get_number_of_ways_record_distance_can_be_broken(
    ) -> Result<(), ParseBoatRacesFromStringError> {
        let input = include_str!("../../example.txt");
        let boat_races = BoatRaceEvent::from_str(input)?;
        let result = boat_races.multiply_number_of_ways_each_race_record_can_be_broken();
        assert_eq!(result, 288);
        Ok(())
    }
}
