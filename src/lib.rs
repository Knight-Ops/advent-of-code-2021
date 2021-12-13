#![feature(hash_drain_filter)]

pub mod day1;
pub mod day10;
pub mod day12;
pub mod day13;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day11;

pub fn read_input_file(input: &str) -> String {
    std::fs::read_to_string(input)
        .expect("Error while reading provided file name")
        .trim()
        .to_string()
}
