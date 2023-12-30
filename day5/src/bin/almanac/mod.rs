use std::collections::VecDeque;

pub mod map;
pub mod range;

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

    fn parse_seed_str(seed_str: &str) -> Result<Vec<u32>, &str> {
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

    fn parse_mappers_str(mappers_str: Vec<&str>) -> Result<Vec<map::Map>, &str> {
        let mut mappers: Vec<map::Map> = vec![];

        for mapper_str in mappers_str.iter() {
            let mapper = map::Map::parse(mapper_str.trim())?;
            mappers.push(mapper);
        }

        Ok(mappers)
    }

    pub fn parse(input: &str) -> Result<Self, &str> {
        let (seed_str, mappers_str) = Self::split_seeds_from_mappers(input)?;
        let seeds = Self::parse_seed_str(seed_str)?;
        let mappers = Self::parse_mappers_str(mappers_str)?;
        Ok(Self { seeds, mappers })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn almanac_parses_successfully() -> Result<(), String> {
        let input = include_str!("../example.txt");
        let almanac = Almanac::parse(input)?;
        dbg!(&almanac);
        assert_eq!(almanac.seeds.len(), 4);
        assert_eq!(almanac.mappers.len(), 7);
        Ok(())
    }
}
