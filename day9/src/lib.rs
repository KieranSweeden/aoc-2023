use std::fs::read_to_string;

#[derive(Debug)]
pub struct Report {
    areas: Vec<Area>,
}

impl Report {
    pub fn get_next_value_area_prediction_sum(&self) -> i32 {
        self.areas
            .iter()
            .map(|area| area.get_next_value_prediction())
            .sum()
    }

    pub fn get_past_value_area_prediction_sum(&self) -> i32 {
        self.areas
            .iter()
            .map(|area| area.get_past_value_prediction())
            .sum()
    }
}

impl TryFrom<&str> for Report {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input.lines().map(|line| Area::try_from(line)).collect() {
            Ok(areas) => Ok(Self { areas }),
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug)]
struct Area {
    history: Vec<Value>,
}

impl Area {
    fn get_next_value_prediction(&self) -> i32 {
        let latest_value = self.history.last().unwrap().0;
        let mut last_nums: Vec<i32> = vec![latest_value];
        let mut diffs: Vec<i32> = self
            .history
            .windows(2)
            .map(|pair| pair[1].0 - pair[0].0)
            .collect();
        last_nums.push(*diffs.last().unwrap());

        while !diffs.iter().all(|diff| *diff == 0) {
            diffs = diffs.windows(2).map(|pair| pair[1] - pair[0]).collect();
            last_nums.push(*diffs.last().unwrap())
        }

        last_nums.iter().sum()
    }

    fn get_history_reversed(&self) -> Vec<Value> {
        let mut history_reversed = self.history.clone();
        history_reversed.reverse();
        history_reversed
    }

    fn get_past_value_prediction(&self) -> i32 {
        let reversed_history = self.get_history_reversed();
        let latest_value = reversed_history.last().unwrap().0;
        let mut last_nums: Vec<i32> = vec![latest_value];
        let mut diffs: Vec<i32> = reversed_history
            .windows(2)
            .map(|pair| pair[1].0 - pair[0].0)
            .collect();
        last_nums.push(*diffs.last().unwrap());

        while !diffs.iter().all(|diff| *diff == 0) {
            diffs = diffs.windows(2).map(|pair| pair[1] - pair[0]).collect();
            last_nums.push(*diffs.last().unwrap())
        }

        last_nums.iter().sum()
    }
}

impl TryFrom<&str> for Area {
    type Error = String;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        match line
            .split_whitespace()
            .map(|value| Value::try_from(value))
            .collect()
        {
            Ok(history) => Ok(Self { history }),
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug, Clone)]
struct Value(i32);

impl TryFrom<&str> for Value {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<i32>() {
            Ok(parsed_value) => Ok(Self(parsed_value)),
            Err(error) => Err(error.to_string()),
        }
    }
}

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
