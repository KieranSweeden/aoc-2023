use std::str::FromStr;

mod camel_cards;

fn process(input: &str) -> Result<u64, camel_cards::error::Error> {
    let game = camel_cards::game::Game::from_str(input)?;
    Ok(game.calculate_winnings())
}

fn main() {
    let input = include_str!("./input.txt");
    match process(input) {
        Ok(result) => println!("The result is: {}", result),
        Err(e) => panic!("Error: {:?}", e),
    }
}
