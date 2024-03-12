mod boat_races;

use boat_races::event::{BoatRaceEvent, ParseBoatRacesFromStringError};
use std::str::FromStr;

fn process(input: &str) -> Result<u32, ParseBoatRacesFromStringError> {
    let boat_races = BoatRaceEvent::from_str(input)?;
    let result = boat_races.multiply_number_of_ways_each_race_record_can_be_broken();
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
