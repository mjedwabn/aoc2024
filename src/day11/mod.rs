use std::io::BufRead;

use crate::read_input;

pub fn count_stones(input: &mut dyn BufRead, blinks: u32) -> usize {
  let mut stones: Vec<u64> = parse_stones(read_input(input));

  for _ in 0..blinks {
    let mut i = 0;
    
    while i < stones.len() {
      let stone = stones[i];

      if stone == 0 {
        stones[i] = 1;
      } else {
        let s = stone.to_string();
        if s.len() % 2 == 0 {
          let (left, right) = s.split_at(s.len() / 2);
          let left_stone = left.parse::<u64>().unwrap();
          let right_stone = right.parse::<u64>().unwrap();

          stones[i] = left_stone;

          if i == stones.len() - 1 {
            stones.push(right_stone);
          } else {
            stones.insert(i + 1, right_stone);
          }

          i += 1;
        } else {
          stones[i] = stones[i] * 2024;
        }
      }

      i += 1;
    }
  }

  stones.len()
}

fn parse_stones(lines: Vec<String>) -> Vec<u64> {
  lines
    .first()
    .map(|line| line.split(' ').flat_map(|s| s.parse::<u64>()).collect())
    .unwrap()
}

#[cfg(test)]
mod tests {
  use crate::{day11::count_stones, read};

  #[test]
  fn sample1_part1_input() {
    assert_eq!(count_stones(&mut read("./src/day11/sample1.input"), 1), 7)
  }

  #[test]
  fn sample2_part1_input() {
    assert_eq!(count_stones(&mut read("./src/day11/sample2.input"), 6), 22);
    assert_eq!(
      count_stones(&mut read("./src/day11/sample2.input"), 25),
      55312
    )
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(count_stones(&mut read("./src/day11/my.input"), 25), 197357)
  }
}
