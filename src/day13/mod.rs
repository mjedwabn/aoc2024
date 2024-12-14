use std::io::BufRead;

use crate::read_input;

pub fn find_minimum_tokens_to_win_possible_prizes(input: &mut dyn BufRead) -> usize {
  parse_claw_machines(read_input(input))
    .iter()
    .flat_map(|machine| find_minimum_tokens(machine))
    .sum()
}

pub fn find_minimum_tokens_to_win_possible_higher_prizes(input: &mut dyn BufRead) -> usize {
  fn higher_prize(machine: &ClawMachine) -> ClawMachine {
    ClawMachine {
      button_a: machine.button_a,
      button_b: machine.button_b,
      prize: (10000000000000 + machine.prize.0, 10000000000000 + machine.prize.1)
    }
  }

  parse_claw_machines(read_input(input)).iter().map(|m| higher_prize(m))
    .flat_map(|machine| find_minimum_tokens(&machine))
    .sum()
}

fn find_minimum_tokens(machine: &ClawMachine) -> Option<usize> {
  let (a, b) = solve_linear_equations(
    (
      machine.button_a.0 as f64,
      machine.button_b.0 as f64,
      machine.prize.0 as f64,
    ),
    (
      machine.button_a.1 as f64,
      machine.button_b.1 as f64,
      machine.prize.1 as f64,
    ),
  );

  if a >= 0f64 && (a as usize) as f64 == a && b >= 0f64 && (b as usize) as f64 == b {
    Some(a as usize * 3 + b as usize)
  } else {
    None
  }
}

fn solve_linear_equations(equation_1: (f64, f64, f64), equation_2: (f64, f64, f64)) -> (f64, f64) {
  solve_linear_quations_using_cramers_formula(equation_1, equation_2)
}

fn solve_linear_quations_using_cramers_formula(equation_1: (f64, f64, f64), equation_2: (f64, f64, f64)) -> (f64, f64) {
  let (a, b, e) = equation_1;
  let (c, d, f) = equation_2;

  let x = (e * d - b * f) / (a * d - b * c);
  let y = (a * f - e * c) / (a * d - b * c);

  (x, y)
}

fn parse_claw_machines(lines: Vec<String>) -> Vec<ClawMachine> {
  fn parse_claw_machine(lines: &[String]) -> ClawMachine {
    let re = regex::Regex::new(r"(?<x>\d+)\D+(?<y>\d+)").unwrap();

    let mut it = lines
      .iter()
      .flat_map(|line| re.captures(line))
      .map(|captures| {
        (
          captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
          captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        )
      });

    ClawMachine {
      button_a: it.next().unwrap(),
      button_b: it.next().unwrap(),
      prize: it.next().unwrap(),
    }
  }

  lines
    .split(|line| line == "")
    .map(|claw_machine| parse_claw_machine(claw_machine))
    .collect()
}

struct ClawMachine {
  button_a: (usize, usize),
  button_b: (usize, usize),
  prize: (usize, usize),
}

#[cfg(test)]
mod tests {
  use crate::{day13::{find_minimum_tokens_to_win_possible_higher_prizes, find_minimum_tokens_to_win_possible_prizes}, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(
      find_minimum_tokens_to_win_possible_prizes(&mut read("./src/day13/sample.input")),
      480
    )
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(
      find_minimum_tokens_to_win_possible_prizes(&mut read("./src/day13/my.input")),
      29438
    )
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(
      find_minimum_tokens_to_win_possible_higher_prizes(&mut read("./src/day13/sample.input")),
      875318608908
    )
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(
      find_minimum_tokens_to_win_possible_higher_prizes(&mut read("./src/day13/my.input")),
      104958599303720
    )
  }
}
