mod almanac;

fn main() {
    let input = include_str!("input.txt");
    match process(input) {
        Ok(result) => println!("The result is {result}"),
        Err(error) => panic!("{error}"),
    }
}

fn process(input: &str) -> Result<i64, &str> {
    let almanac = almanac::Almanac::parse(input)?;
    Ok(almanac.get_closest_location())
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
