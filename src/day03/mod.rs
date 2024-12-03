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

#[cfg(test)]
mod tests {
  use crate::day03::sum_multiplications;
  use std::{fs::File, io::BufReader};

  #[test]
  fn sample_part1_input() {
    let mut f = BufReader::new(File::open("./src/day03/sample.input").unwrap());
    assert_eq!(sum_multiplications(&mut f), 161)
  }

  #[test]
  fn my_part1_input() {
    let mut f = BufReader::new(File::open("./src/day03/my.input").unwrap());
    assert_eq!(sum_multiplications(&mut f), 174960292)
  }
}
