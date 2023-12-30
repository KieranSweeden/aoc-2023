const DESTINATION_START_INDEX: usize = 0;
const SOURCE_START_INDEX: usize = 1;
const RANGE_INDEX: usize = 2;

#[derive(Debug)]
pub struct SourceRange {
    pub start: u32,
    pub end: u32,
    pub destination_difference: i32,
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

#[cfg(test)]
mod test {
    use super::*;

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
