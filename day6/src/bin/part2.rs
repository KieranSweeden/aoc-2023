mod boat_races;

use boat_races::event::{ParseBoatRacesFromStringError, SingleRaceBoatRaceEvent};
use std::str::FromStr;

fn process(input: &str) -> Result<u64, ParseBoatRacesFromStringError> {
    let event = SingleRaceBoatRaceEvent::from_str(input)?;
    let result = event
        .race
        .get_number_of_ways_record_distance_can_be_broken();
    Ok(result)
}

fn main() {
    let input = include_str!("../input.txt");
    let result = process(input);
    match result {
        Ok(res) => println!("The result is: {}", res),
        Err(e) => eprintln!("An error occured: {:?}", e),
    }
}
