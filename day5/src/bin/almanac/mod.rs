use std::collections::VecDeque;

pub mod map;
pub mod range;

pub enum SeedParseMode {
    Simple,
    Range,
}

pub struct AlmanacOptions {
    pub seed_parse_mode: SeedParseMode,
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u32>,
    mappers: Vec<map::Map>,
}

impl Almanac {
    fn split_seeds_from_mappers(input: &str) -> Result<(&str, Vec<&str>), &str> {
        let mut parts: VecDeque<&str> = input.split("\n\n").collect();
        let seed_str = parts
            .pop_front()
            .expect("Failed to retrieve seeds part from input");

        Ok((seed_str, parts.into()))
    }

    fn parse_seed_str_simple(seed_str: &str) -> Result<Vec<u32>, &str> {
        let seed_id_list_str = seed_str
            .split(':')
            .nth(1)
            .expect("Failed to retrieve seed ids");

        Ok(seed_id_list_str
            .trim()
            .split_whitespace()
            .map(|seed_id| {
                seed_id
                    .parse::<u32>()
                    .expect("Failed to parse seed id to number")
            })
            .collect())
    }

    fn parse_seed_str_range(seed_str: &str) -> Result<Vec<u32>, &str> {
        let seed_id_list_str: Vec<u32> = seed_str
            .split(':')
            .nth(1)
            .expect("Failed to retrieve seed ids")
            .trim()
            .split_whitespace()
            .map(|seed_id| seed_id.parse::<u32>().expect("Failed"))
            .collect();

        let mut seed_id_list: Vec<u32> = vec![];

        for [start, range] in seed_id_list_str.chunks(2) {
            for i in *start..(*start + *range) {}
        }

        Ok(seed_id_list)
    }

    fn parse_mappers_str(mappers_str: Vec<&str>) -> Result<Vec<map::Map>, &str> {
        let mut mappers: Vec<map::Map> = vec![];

        for mapper_str in mappers_str.iter() {
            let mapper = map::Map::parse(mapper_str.trim())?;
            mappers.push(mapper);
        }

        Ok(mappers)
    }

    pub fn get_closest_location(&self) -> i64 {
        let mut closest_location_id: i64 = 0;

        for seed_source_id in self.seeds.iter() {
            let location_id = self.get_location_for_seed(*seed_source_id);
            if closest_location_id == 0 {
                closest_location_id = location_id;
            } else {
                if location_id < closest_location_id {
                    closest_location_id = location_id;
                }
            }
        }

        closest_location_id
    }

    fn get_location_for_seed(&self, first_source_id: u32) -> i64 {
        self.mappers
            .iter()
            .fold(first_source_id.into(), |source_id, mapper| {
                mapper.get_destination_id_by_source_id(source_id as u32)
            })
    }

    pub fn parse(input: &str, options: AlmanacOptions) -> Result<Self, &str> {
        let (seed_str, mappers_str) = Self::split_seeds_from_mappers(input)?;
        let seeds = match options.seed_parse_mode {
            SeedParseMode::Simple => Self::parse_seed_str_simple(seed_str)?,
            SeedParseMode::Range => Self::parse_seed_str_range(seed_str)?,
        };
        let mappers = Self::parse_mappers_str(mappers_str)?;
        Ok(Self { seeds, mappers })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_location_for_seed() -> Result<(), &'static str> {
        let input = include_str!("../example.txt");
        let options = AlmanacOptions {
            seed_parse_mode: SeedParseMode::Simple,
        };
        let almanac = Almanac::parse(input, options)?;
        assert_eq!(almanac.get_location_for_seed(79), 82);
        assert_eq!(almanac.get_location_for_seed(14), 43);
        assert_eq!(almanac.get_location_for_seed(55), 86);
        assert_eq!(almanac.get_location_for_seed(13), 35);
        Ok(())
    }

    #[test]
    fn almanac_parses_successfully() -> Result<(), &'static str> {
        let input = include_str!("../example.txt");
        let options = AlmanacOptions {
            seed_parse_mode: SeedParseMode::Simple,
        };
        let almanac = Almanac::parse(input, options)?;
        assert_eq!(almanac.seeds.len(), 4);
        assert_eq!(almanac.mappers.len(), 7);
        Ok(())
    }
}
