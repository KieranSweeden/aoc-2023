mod scratchcard;

fn process(input: &str) -> Result<u32, &str> {
    let scratchcard_pile = scratchcard::pile::Pile::parse(input)?;
    Ok(scratchcard_pile.calculate_points())
}

fn main() {
    let input = include_str!("input.txt");
    let result = process(input);
    match result {
        Ok(result) => println!("The result is {result}"),
        Err(error) => panic!("{error}"),
    }
}
