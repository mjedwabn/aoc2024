use std::{collections::HashMap, io::BufRead};

use itertools::Itertools;

use crate::read_input;

pub fn count_stones(input: &mut dyn BufRead, blinks: u32) -> usize {
  let mut stones: HashMap<u64, usize> = parse_stones(read_input(input));

  for _ in 0..blinks {
    let mut tmp: HashMap<u64, isize> = HashMap::new();

    for (stone, n) in stones.iter() {
      let (left, right) = if *stone == 0 {
        (1, None)
      } else {
        let s = stone.to_string();
        if s.len() % 2 == 0 {
          let (left, right) = s.split_at(s.len() / 2);
          (
            left.parse::<u64>().unwrap(),
            Some(right.parse::<u64>().unwrap()),
          )
        } else {
          (stone * 2024, None)
        }
      };

      tmp
        .entry(*stone)
        .and_modify(|count| *count = *count - *n as isize)
        .or_insert(-(*n as isize));
      
      tmp
        .entry(left)
        .and_modify(|count| *count = *count + *n as isize)
        .or_insert(*n as isize);
      
      if let Some(r) = right {
        tmp
          .entry(r)
          .and_modify(|count| *count = *count + *n as isize)
          .or_insert(*n as isize);
      }
    }

    for (stone, n) in tmp {
      stones
        .entry(stone)
        .and_modify(|count| *count = (*count as isize + n) as usize)
        .or_insert(n as usize);
    }
  }

  stones.values().sum()
}

fn parse_stones(lines: Vec<String>) -> HashMap<u64, usize> {
  lines
    .first()
    .map(|line| {
      line
        .split(' ')
        .flat_map(|s| s.parse::<u64>())
        .into_group_map_by(|&s| s)
        .into_iter()
        .map(|s| (s.0, s.1.len()))
        .collect()
    })
    .unwrap()
}

#[cfg(test)]
mod tests {
  use crate::{day11::count_stones, read};

  #[test]
  fn sample1_part1_input() {
    assert_eq!(count_stones(&mut read("./src/day11/sample1.input"), 1), 7);
  }

  #[test]
  fn sample2_part1_input() {
    assert_eq!(count_stones(&mut read("./src/day11/sample2.input"), 6), 22);
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(count_stones(&mut read("./src/day11/my.input"), 25), 197357);
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(
      count_stones(&mut read("./src/day11/my.input"), 75),
      234568186890978
    )
  }
}
