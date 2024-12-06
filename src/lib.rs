use std::io::BufRead;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

pub fn read_input(input: &mut dyn BufRead) -> Vec<String> {
    return input.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
}