fn process(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut part_numbers: Vec<u32> = vec![];
    let mut active_number: Option<String> = None;

    for (line_index, line) in lines.iter().enumerate() {
        if active_number.is_some() {
            println!("Previous line's active number is not part number, clearing..");
            active_number = None;
        }

        'char_loop: for (char_index, character) in line.char_indices() {
            if character.is_ascii_digit() {
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

                if char_index != line.len() - 1 {
                    continue;
                }
            }

            if let Some(ref mut active_digit) = active_number.clone() {
                println!("Finished reading active number {active_digit}, accessing if its a valid part number");

                dbg!(char_index);

                let start_index = if character.is_ascii_digit() {
                    char_index - (active_digit.len() - 1)
                } else {
                    char_index - active_digit.len()
                };

                dbg!(start_index);

                let end_index = start_index + active_digit.len() - 1;

                dbg!(end_index);

                let left_index = start_index.checked_sub(1);
                let right_index = if end_index == line.len() - 1 {
                    None
                } else {
                    Some(end_index + 1)
                };

                dbg!(right_index);

                // sort out external line range indices
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

                println!("External line range is {external_line_start_index} to {external_line_end_index}");

                // check line above
                if let Some(line_above_index) = line_index.checked_sub(1) {
                    let line_above = lines.get(line_above_index).expect(
                        "Failed to retrieve line above despite having checked Some(index) to do so",
                    );

                    println!("Line above found");

                    for character in line_above
                        .chars()
                        .skip(external_line_start_index)
                        .take((external_line_end_index - external_line_start_index) + 1)
                    {
                        if !character.is_ascii_digit() && character != '.' {
                            println!("SYMBOL!");
                            let parsed_digit: u32 = active_digit
                                .parse()
                                .expect("Failed to parse stringified number to u32");

                            part_numbers.push(parsed_digit);
                            active_number = None;

                            println!("Skipping line above loop and continuing with char loop");
                            continue 'char_loop;
                        }
                    }
                };

                // check left & right on current line
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

                        part_numbers.push(parsed_digit);
                        active_number = None;
                        continue;
                    }
                }

                if let Some(index) = right_index {
                    dbg!(line);
                    dbg!(line.len());
                    dbg!(index);

                    let character = line
                        .chars()
                        .nth(index)
                        .expect("Failed to retrieve right indexed number on current line");

                    if !character.is_ascii_digit() && character != '.' {
                        println!("SYMBOL!");
                        let parsed_digit: u32 = active_digit
                            .parse()
                            .expect("Failed to parse stringified number to u32");

                        part_numbers.push(parsed_digit);
                        active_number = None;
                        continue;
                    }
                }

                // check line below
                if let Some(line_below) = lines.get(line_index + 1) {
                    println!("Line below found");

                    for character in line_below
                        .chars()
                        .skip(external_line_start_index)
                        .take((external_line_end_index - external_line_start_index) + 1)
                    {
                        if !character.is_ascii_digit() && character != '.' {
                            println!("SYMBOL!");

                            let parsed_digit: u32 = active_digit
                                .parse()
                                .expect("Failed to parse stringified number to u32");

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

    dbg!(&part_numbers);

    part_numbers.iter().sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let result = process(input);
    println!("The result is: {result}");
}

#[cfg(test)]
mod part_1_tests {
    use super::*;

    #[test]
    fn process_passes() {
        let input = include_str!("./sample.txt");
        let sum = process(input);
        assert_eq!(sum, 4361 as u32);
    }
}
