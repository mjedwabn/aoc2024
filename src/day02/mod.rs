use std::io::BufRead;

use crate::read_input;

pub fn count_safe_reports(input: &mut dyn BufRead) -> usize {
  let reports = parse_input(read_input(input));
  reports.iter().filter(|r| is_report_safe(r)).count()
}

pub fn count_safe_reports_with_toleration(input: &mut dyn BufRead) -> usize {
  let reports = parse_input(read_input(input));
  reports.iter().filter(|r| is_report_safe_with_toleration(r)).count()
}

fn is_report_safe_with_toleration(report: &Vec<u32>) -> bool {
  is_report_safe(report)
    || (0..report.len()).any(|level: usize| is_report_safe_without_level(report, level))
}

fn is_report_safe_without_level(report: &Vec<u32>, level: usize) -> bool {
  fn without_level(report: &Vec<u32>, level: usize) -> Vec<u32> {
    report
      .iter()
      .enumerate()
      .filter(|&(i, _)| i != level)
      .map(|(_, v)| *v)
      .collect::<Vec<u32>>()
  }

  is_report_safe(&without_level(report, level))
}

fn is_report_safe(report: &Vec<u32>) -> bool {
  are_levels_monothonic(report, |d| d >= 1 && d <= 3)
    || are_levels_monothonic(report, |d| d <= -1 && d >= -3)
}

fn are_levels_monothonic(report: &Vec<u32>, f: fn(i32) -> bool) -> bool {
  report
    .windows(2)
    .map(|w| w[0] as i32 - w[1] as i32)
    .all(|d| f(d))
}

fn parse_input(input: Vec<String>) -> Vec<Vec<u32>> {
  fn parse_line(line: &String) -> Vec<u32> {
    line
      .split(' ')
      .map(|c| c.parse::<u32>().unwrap())
      .collect::<Vec<u32>>()
  }

  input
    .iter()
    .map(|line| parse_line(line))
    .collect::<Vec<Vec<u32>>>()
}

#[cfg(test)]
mod tests {
  use crate::day02::{count_safe_reports, count_safe_reports_with_toleration};
  use std::{fs::File, io::BufReader};

  #[test]
  fn sample_part1_input() {
    let mut f = BufReader::new(File::open("./src/day02/sample.input").unwrap());
    assert_eq!(count_safe_reports(&mut f), 2);
  }

  #[test]
  fn my_part1_input() {
    let mut f = BufReader::new(File::open("./src/day02/my.input").unwrap());
    assert_eq!(count_safe_reports(&mut f), 390);
  }

  #[test]
  fn sample_part2_input() {
    let mut f = BufReader::new(File::open("./src/day02/sample.input").unwrap());
    assert_eq!(count_safe_reports_with_toleration(&mut f), 4);
  }

  #[test]
  fn my_part2_input() {
    let mut f = BufReader::new(File::open("./src/day02/my.input").unwrap());
    assert_eq!(count_safe_reports_with_toleration(&mut f), 439);
  }
}
