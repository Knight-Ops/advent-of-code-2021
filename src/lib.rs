pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub fn read_input_file(input: &str) -> String {
    std::fs::read_to_string(input)
        .expect("Error while reading provided file name")
        .trim()
        .to_string()
}
