mod node;

fn process(input: &str) -> Result<u32, String> {
    let lines: Vec<&str> = input.lines().collect();

    let instructions = node::Instructions::new(lines[0]).expect("Failed to parse instructions");
    let mut network = node::Network::new();

    for line in lines.iter().skip(2) {
        let node = node::Node::new(&line[..3], &line[7..10], &line[12..15]);
        network.add_node(node);
    }

    let result = network
        .calculate_steps_to_zzz(&instructions)
        .expect("failed");

    Ok(result)
}

fn main() {
    let input = include_str!("./input.txt");
    match process(input) {
        Ok(result) => println!("The result is {result}"),
        Err(err) => panic!("{err}"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn process_succeeds() {
        let input = include_str!("./example.txt");
        let result = process(input);
    }
}
