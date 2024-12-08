use std::io::BufRead;

use crate::read_input;

pub fn total_calibration_result(input: &mut dyn BufRead, possible_operators: &[char]) -> u64 {
  parse_equations(read_input(input))
    .iter()
    .filter(|e| e.can_be_made_true(possible_operators))
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
    fn can_be_made_true(&self, possible_operators: &[char]) -> bool {
      self.operator_combinations(possible_operators).iter()
        .map(|s| s.chars().collect())
        .any(|operators| self.eval(operators) == self.test_value)
    }

    fn operator_combinations(&self, ops: &[char]) -> Vec<String> {
      let n = self.numbers.len() - 1;
      (0..n).fold(vec![String::from("+")], |acc, _| {
          acc.into_iter()
              .flat_map(|head| ops.iter().map(move |&o| format!("{head}{o}")))
              .collect()
      })
    }

    fn eval(&self, operators: Vec<char>) -> u64 {
      self.numbers.iter().zip(operators)
        .fold(0, |acc, e| if e.1 == '+' {acc + e.0} else if e.1 == '|' { self.concat(acc, *e.0) } else {acc * e.0})
    }

    fn concat(&self, a: u64, b: u64) -> u64 {
      format!("{a}{b}").parse::<u64>().unwrap()
    }
}

#[cfg(test)]
mod tests {
  use crate::{day07::total_calibration_result, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(total_calibration_result(&mut read("./src/day07/sample.input"), &vec!['+', '*']), 3749)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(total_calibration_result(&mut read("./src/day07/my.input"), &vec!['+', '*']), 12839601725877)
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(total_calibration_result(&mut read("./src/day07/sample.input"), &vec!['+', '*', '|']), 11387)
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(total_calibration_result(&mut read("./src/day07/my.input"), &vec!['+', '*', '|']), 149956401519484)
  }
}