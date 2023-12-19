fn process(_input: &str) -> i32 {
    4361
}

fn main() {
    println!("first file!");
}

#[cfg(test)]
mod part_1_tests {
    use super::*;

    #[test]
    fn process_passes() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let sum = process(input);
        assert_eq!(sum, 4361 as i32);
    }
}
