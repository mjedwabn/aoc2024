use std::io::BufRead;

use itertools::Itertools;

pub fn what_is_the_total_distance_between_lists(input: &mut dyn BufRead) -> u32 {
    let (mut left, mut right) = parse_input(read_input(input));
    left.sort();
    right.sort();

    return left.iter().zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();
}

fn read_input(input: &mut dyn BufRead) -> Vec<String> {
    return input.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
}

fn parse_input(input: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let re = regex::Regex::new(r"\s+").unwrap();

    let parsed_lines = input.iter()
        .map(|s| re.splitn(s, 2).collect_tuple().unwrap())
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .collect::<Vec<(u32, u32)>>();

    return (
        parsed_lines.iter().map(|e| e.0).collect::<Vec<u32>>(),
        parsed_lines.iter().map(|e| e.1).collect::<Vec<u32>>()
    );
}

#[cfg(test)]
mod tests {
    use crate::day01::what_is_the_total_distance_between_lists;
    use std::{fs::File, io::BufReader};

    #[test]
    fn sample_part1_input() {
        let mut f = BufReader::new(File::open("./src/day01/sample.input").unwrap());
        assert_eq!(what_is_the_total_distance_between_lists(&mut f), 11);
    }

    #[test]
    fn part1_input() {
        let mut f = BufReader::new(File::open("./src/day01/my.input").unwrap());
        assert_eq!(what_is_the_total_distance_between_lists(&mut f), 1579939);
    }
}
