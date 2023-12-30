use crate::almanac::range::SourceRange;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Map {
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

    pub fn parse(input: &str) -> Result<Self, &str> {
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
}
