use std::io::BufRead;

use crate::read_input;

pub fn sum_multiplications(input: &mut dyn BufRead) -> u32 {
  let muls = extract_valid_muls(read_input(input).concat());
  muls.iter().map(|(a, b)| a * b).sum()
}

fn extract_valid_muls(code: String) -> Vec<(u32, u32)> {
  regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap()
    .captures_iter(&code)
    .map(|c| c.extract().1)
    .map(|[a, b]| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
    .collect::<Vec<(u32, u32)>>()
}

pub fn sum_enabled_multiplications(input: &mut dyn BufRead) -> u32 {
  let muls = extract_enabled_muls(read_input(input).concat());
  muls.iter().map(|(a, b)| a * b).sum()
}

fn extract_enabled_muls(code: String) -> Vec<(u32, u32)> {
  let re = regex::Regex::new(r"(?<disable>don't\(\))|(?<enable>do\(\))|mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
  
  let mut enabled = true;
  let mut enabled_muls: Vec<(u32, u32)> = Vec::new();
  
  for x in re.captures_iter(&code) {
    if x.name("disable").is_some() {
      enabled = false;
    }
    else if x.name("enable").is_some() {
      enabled = true;
    }
    else {
      if enabled {
        enabled_muls.push((
          x.name("a").unwrap().as_str().parse::<u32>().unwrap(), 
          x.name("b").unwrap().as_str().parse::<u32>().unwrap()
        ))
      }
    }
  }

  enabled_muls
}

#[cfg(test)]
mod tests {
  use crate::{day03::{sum_enabled_multiplications, sum_multiplications}, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(sum_multiplications(&mut read("./src/day03/sample.input")), 161)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(sum_multiplications(&mut read("./src/day03/my.input")), 174960292)
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(sum_enabled_multiplications(&mut "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".as_bytes()), 48)
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(sum_enabled_multiplications(&mut read("./src/day03/my.input")), 56275602)
  }
}
