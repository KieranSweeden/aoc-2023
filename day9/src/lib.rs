use std::fs::read_to_string;

#[allow(dead_code)]
fn highest_number_in_file(path: &str) -> Result<i32, String> {
    let file_contents = match read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => return Err(format!("Couldn't find file: {}", path)),
    };

    let max = file_contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    Ok(max)
}

#[allow(dead_code)]
fn lowest_number_in_file(path: &str) -> Result<i32, String> {
    let file_contents = match read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => return Err(format!("Couldn't find file: {}", path)),
    };

    let min = file_contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    Ok(min)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gets_highest_number_in_file() {
        let result = highest_number_in_file("./example.txt");
        assert_eq!(result, Ok(45));

        let result = highest_number_in_file("./input.txt");
        assert_eq!(result, Ok(21793992));
    }

    #[test]
    fn gets_lowest_number_in_file() {
        let result = lowest_number_in_file("./example.txt");
        assert_eq!(result, Ok(0));

        let result = lowest_number_in_file("./input.txt");
        assert_eq!(result, Ok(-1698921));
    }
}
