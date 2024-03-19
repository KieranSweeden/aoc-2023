const DESTINATION_START_INDEX: usize = 0;
const SOURCE_START_INDEX: usize = 1;
const RANGE_INDEX: usize = 2;

pub struct SeedRange {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug)]
pub struct SourceRange {
    pub start: u32,
    pub end: u32,
    pub destination_difference: i64,
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

    pub fn convert_to_destination(&self, source_id: u32) -> i64 {
        source_id as i64 + self.destination_difference as i64
    }

    pub fn contains(&self, source_id: u32) -> bool {
        source_id >= self.start && source_id <= self.end
    }

    pub fn parse(input: &str) -> Result<Self, &str> {
        let (destination_start_id, source_start_id, range) = Self::get_numbers(input)?;

        Ok(Self {
            start: source_start_id,
            end: source_start_id + (range - 1),
            destination_difference: destination_start_id as i64 - source_start_id as i64,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn source_ranges_contains_passes() -> Result<(), &'static str> {
        let input = "50 98 2";
        let source_range = SourceRange::parse(input)?;

        assert_eq!(source_range.convert_to_destination(99), 51);
        assert_eq!(source_range.convert_to_destination(98), 50);

        Ok(())
    }

    #[test]
    fn source_ranges_destination_conversion_passes() -> Result<(), &'static str> {
        let input = "50 98 2";
        let source_range = SourceRange::parse(input)?;

        assert_eq!(source_range.contains(99), true);
        assert_eq!(source_range.contains(21), false);
        assert_eq!(source_range.contains(98), true);

        Ok(())
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
