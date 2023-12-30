use std::collections::VecDeque;

fn main() {
    println!("Hello, part 1!");
}

const DESTINATION_START_INDEX: usize = 0;
const SOURCE_START_INDEX: usize = 1;
const RANGE_INDEX: usize = 2;

#[derive(Debug)]
struct SourceRange {
    start: u32,
    end: u32,
    destination_difference: i32,
}

impl SourceRange {
    fn parse_number(number: &str) -> Result<u32, &str> {
        match number.parse::<u32>() {
            Ok(number) => Ok(number),
            Err(_) => Err("Failed to parse into number string to u32 int"),
        }
    }

    fn get_numbers(input: &str) -> Result<(u32, u32, u32), &str> {
        let numbers: Vec<&str> = input.split_whitespace().collect();
        let destination_start_id = Self::parse_number(
            numbers
                .get(DESTINATION_START_INDEX)
                .expect("Failed to extract destination start id from input"),
        )?;

        let source_start_id = Self::parse_number(
            numbers
                .get(SOURCE_START_INDEX)
                .expect("Failed to extract source start id from input"),
        )?;

        let range = Self::parse_number(
            numbers
                .get(RANGE_INDEX)
                .expect("Failed to extract source start id from input"),
        )?;

        Ok((destination_start_id, source_start_id, range))
    }

    pub fn parse(input: &str) -> Result<Self, &str> {
        let (destination_start_id, source_start_id, range) = Self::get_numbers(input)?;

        Ok(Self {
            start: source_start_id,
            end: source_start_id + (range - 1),
            destination_difference: destination_start_id as i32 - source_start_id as i32,
        })
    }
}

#[derive(Debug)]
struct Map {
    source_name: String,
    destination_name: String,
    source_ranges: Vec<SourceRange>,
}

impl Map {
    fn parse_names_from_description(line: &str) -> Result<(String, String), &str> {
        let type_to_type_part = line
            .trim()
            .split_whitespace()
            .nth(0)
            .expect("Failed to retrieve type-to-type part from description");

        let types: Vec<&str> = type_to_type_part.split("-to-").collect();
        let source_type_name = types.get(0).expect("Failed to retrieve source type name");
        let destination_type_name = types
            .get(1)
            .expect("Failed to retrieve destination type name");

        Ok((
            String::from(*source_type_name),
            String::from(*destination_type_name),
        ))
    }

    fn parse(input: &str) -> Result<Self, &str> {
        let mut lines: VecDeque<&str> = input.lines().collect();
        let description = lines
            .pop_front()
            .expect("Failed to retrieve description string");

        let (source_name, destination_name) = Self::parse_names_from_description(description)?;

        let mut source_ranges: Vec<SourceRange> = vec![];

        for source_range_line in lines.iter() {
            let source_range = SourceRange::parse(source_range_line)?;
            source_ranges.push(source_range);
        }

        Ok(Self {
            source_name,
            destination_name,
            source_ranges,
        })
    }

    fn get_destination_id_by_source_id(&self, source_id: u32) -> i32 {
        let mut destination_id = source_id as i32;

        for source_range in self.source_ranges.iter() {
            if (source_range.start..=source_range.end).contains(&source_id) {
                destination_id = destination_id + source_range.destination_difference;
                break;
            }
        }

        destination_id
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_returns_correct_destination_id() {
        let input = "seed-to-soil map:
50 98 2
52 50 48";
        let map = Map::parse(input);
        match map {
            Ok(map) => {
                assert_eq!(map.get_destination_id_by_source_id(56), 58);
                assert_eq!(map.get_destination_id_by_source_id(99), 51);
            }
            Err(error) => panic!("{error}"),
        }
    }

    #[test]
    fn map_parses_successfully() {
        let input = "seed-to-soil map:
50 98 2
52 50 48";
        let map = Map::parse(input);
        dbg!(&map);
        match map {
            Ok(map) => {
                assert_eq!(map.source_name, "seed");
                assert_eq!(map.destination_name, "soil");
                assert_eq!(map.source_ranges.len(), 2);
            }
            Err(error) => panic!("{error}"),
        }
    }

    #[test]
    fn source_range_parses_successfully() {
        let input = "50 98 2";
        let source_range = SourceRange::parse(input);
        match source_range {
            Ok(source_range) => {
                assert_eq!(source_range.start, 98);
                assert_eq!(source_range.end, 99);
                assert_eq!(source_range.destination_difference, -48);
            }
            Err(error) => panic!("{error}"),
        }

        let input = "52 50 48";
        let source_range = SourceRange::parse(input);
        match source_range {
            Ok(source_range) => {
                assert_eq!(source_range.start, 50);
                assert_eq!(source_range.end, 97);
                assert_eq!(source_range.destination_difference, 2);
            }
            Err(error) => panic!("{error}"),
        }
    }
}
