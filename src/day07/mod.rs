use std::io::BufRead;

use crate::read_input;

pub fn total_calibration_result(input: &mut dyn BufRead) -> u64 {
  parse_equations(read_input(input))
    .iter()
    .filter(|e| e.can_be_made_true())
    .map(|e| e.test_value)
    .sum()
}

fn parse_equations(lines: Vec<String>) -> Vec<Equation> {
  fn parse_equation(line: &String) -> Equation {
    line.split_once(':')
      .map(|p| Equation {
        test_value: p.0.parse::<u64>().unwrap(), 
        numbers: p.1.trim().split(' ').map(|n| n.parse::<u64>().unwrap()).collect()
      })
      .unwrap()
  }

  lines.iter().map(|line| parse_equation(line)).collect()
}

struct Equation {
  test_value: u64,
  numbers: Vec<u64>
}

impl Equation {
    fn can_be_made_true(&self) -> bool {
      self.operators_combinations()
        .any(|operators| self.eval(operators) == self.test_value)
    }

    fn operators_combinations(&self) -> impl Iterator<Item = Vec<char>> {
      (0..(2 as u32).pow((self.numbers.len() - 1) as u32))
        .map(|i| format!("0{:0n$b}", i, n=self.numbers.len() - 1))
        .map(|binary| binary.chars().collect::<Vec<char>>())
    }

    fn eval(&self, operators: Vec<char>) -> u64 {
      self.numbers.iter().zip(operators)
        .fold(0, |acc, e| if e.1 == '0' {acc + e.0} else {acc * e.0})
    }
}

#[cfg(test)]
mod tests {
  use std::{fs::File, io::BufReader};

use crate::day07::total_calibration_result;

  #[test]
  fn sample_part1_input() {
    let mut f = BufReader::new(File::open("./src/day07/sample.input").unwrap());
    assert_eq!(total_calibration_result(&mut f), 3749)
  }

  #[test]
  fn my_part1_input() {
    let mut f = BufReader::new(File::open("./src/day07/my.input").unwrap());
    assert_eq!(total_calibration_result(&mut f), 12839601725877)
  }
}