use day9::Report;

fn main() {
    let input = include_str!("../../input.txt");
    match Report::try_from(input) {
        Ok(report) => {
            let result = report.get_next_value_area_prediction_sum();
            println!("The result is: {}", result);
        }
        Err(error) => {
            panic!("{error}")
        }
    }
}
