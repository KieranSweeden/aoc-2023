use std::collections::HashMap;

fn process(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut part_numbers: Vec<u32> = vec![];
    let mut active_number: Option<String> = None;
    let mut gear_map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();

    for (line_index, line) in lines.iter().enumerate() {
        // clear previously active non-part number from previous line if existing
        if active_number.is_some() {
            println!("Previous line's active number is not part number, clearing..");
            active_number = None;
        }

        'char_loop: for (char_index, character) in line.char_indices() {
            if character.is_ascii_digit() {
                // digit found, ensure it's a part of the current active number
                match active_number {
                    Some(ref mut active_num) => {
                        println!("Adding {character} to active number {active_num}");
                        active_num.push(character);
                    }
                    None => {
                        println!("Creating new to active number from {character}");
                        active_number = Some(character.to_string());
                    }
                }

                // skip to next char if not last index and is number
                if char_index != line.len() - 1 {
                    continue;
                }
            }

            // check if we've reached a non-digit whilst having an active number in play
            if let Some(ref mut active_digit) = active_number.clone() {
                println!("Finished reading active number {active_digit}, assessing if its a valid part number");

                // define active number index range
                let start_index = if character.is_ascii_digit() {
                    char_index - (active_digit.len() - 1)
                } else {
                    char_index - active_digit.len()
                };
                let end_index = start_index + active_digit.len() - 1;

                // define current line adjacent indexes
                let left_index = start_index.checked_sub(1);
                let right_index = if end_index == line.len() - 1 {
                    None
                } else {
                    Some(end_index + 1)
                };

                // define external line range indices
                let external_line_start_index = if let Some(index) = left_index {
                    index
                } else {
                    start_index
                };
                let external_line_end_index = if let Some(index) = right_index {
                    index
                } else {
                    end_index
                };

                // do symbol find in line above
                if let Some(line_above_index) = line_index.checked_sub(1) {
                    let line_above = lines.get(line_above_index).expect(
                        "Failed to retrieve line above despite having checked Some(index) to do so",
                    );

                    println!("Line above found");

                    for (char_index, character) in line_above
                        .char_indices()
                        .skip(external_line_start_index)
                        .take((external_line_end_index - external_line_start_index) + 1)
                    {
                        if !character.is_ascii_digit() && character != '.' {
                            let parsed_digit: u32 = active_digit
                                .parse()
                                .expect("Failed to parse stringified number to u32");

                            if character == '*' {
                                let key = (char_index as u32, line_above_index as u32);
                                if let Some(part_number_collection) = gear_map.get_mut(&key) {
                                    part_number_collection.push(parsed_digit);
                                } else {
                                    gear_map.insert(key, vec![parsed_digit]);
                                }
                            }

                            part_numbers.push(parsed_digit);
                            active_number = None;

                            println!("Skipping line above loop and continuing with char loop");
                            continue 'char_loop;
                        }
                    }
                };

                // do symbol find on left position of current line (if index exists)
                if let Some(index) = left_index {
                    let character = line
                        .chars()
                        .nth(index)
                        .expect("Failed to retrieve left indexed number on current line");

                    if !character.is_ascii_digit() && character != '.' {
                        println!("SYMBOL!");
                        let parsed_digit: u32 = active_digit
                            .parse()
                            .expect("Failed to parse stringified number to u32");

                        if character == '*' {
                            let key = (index as u32, line_index as u32);
                            if let Some(part_number_collection) = gear_map.get_mut(&key) {
                                part_number_collection.push(parsed_digit);
                            } else {
                                gear_map.insert(key, vec![parsed_digit]);
                            }
                        }

                        part_numbers.push(parsed_digit);
                        active_number = None;
                        continue;
                    }
                }

                // do symbol find on left position of current line (if index exists)
                if let Some(index) = right_index {
                    let character = line
                        .chars()
                        .nth(index)
                        .expect("Failed to retrieve right indexed number on current line");

                    if !character.is_ascii_digit() && character != '.' {
                        println!("SYMBOL!");
                        let parsed_digit: u32 = active_digit
                            .parse()
                            .expect("Failed to parse stringified number to u32");

                        let key = (index as u32, line_index as u32);
                        if let Some(part_number_collection) = gear_map.get_mut(&key) {
                            part_number_collection.push(parsed_digit);
                        } else {
                            gear_map.insert(key, vec![parsed_digit]);
                        }

                        part_numbers.push(parsed_digit);
                        active_number = None;
                        continue;
                    }
                }

                // do symbol find in line below
                if let Some(line_below) = lines.get(line_index + 1) {
                    println!("Line below found");

                    for (char_index, character) in line_below
                        .char_indices()
                        .skip(external_line_start_index)
                        .take((external_line_end_index - external_line_start_index) + 1)
                    {
                        if !character.is_ascii_digit() && character != '.' {
                            let parsed_digit: u32 = active_digit
                                .parse()
                                .expect("Failed to parse stringified number to u32");

                            if character == '*' {
                                let key = (char_index as u32, (line_index + 1) as u32);
                                if let Some(part_number_collection) = gear_map.get_mut(&key) {
                                    part_number_collection.push(parsed_digit);
                                } else {
                                    gear_map.insert(key, vec![parsed_digit]);
                                }
                            }

                            part_numbers.push(parsed_digit);
                            active_number = None;

                            println!("Skipping line below loop and continuing with char loop");
                            continue 'char_loop;
                        }
                    }
                };

                println!("Resetting active number");
                active_number = None;
            }
        }
    }

    let gear_ratio_sum: u32 = gear_map
        .into_values()
        .filter_map(|part_number_collection| {
            if part_number_collection.len() == 2 {
                Some(part_number_collection.iter().fold(0, |acc, part_number| {
                    if acc == 0 {
                        acc + part_number
                    } else {
                        acc * part_number
                    }
                }))
            } else {
                None
            }
        })
        .sum();

    gear_ratio_sum
}

fn main() {
    let input = include_str!("./input.txt");
    let result = process(input);
    println!("The result is: {result}");
}

#[cfg(test)]
mod part_2_tests {
    use super::*;

    #[test]
    fn process_passes() {
        let input = include_str!("./sample.txt");
        let sum = process(input);
        assert_eq!(sum, 467835 as u32);
    }
}
