mod almanac;

fn main() {
    println!("Hello, part 1!");
}

fn process(input: &str) -> Result<u32, &str> {
    let almanac = almanac::Almanac::parse(input)?;
    Ok(almanac.get_nearest_location_id())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn process_passes() -> Result<(), &'static str> {
        let input = include_str!("example.txt");
        let result = process(input)?;
        assert_eq!(result, 35);
        Ok(())
    }
}
