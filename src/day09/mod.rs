use std::io::BufRead;

use itertools::Itertools;

use crate::read_input;

const FREE: i32 = -1;

pub fn checksum(input: &mut dyn BufRead) -> u64 {
  let disk_map = parse_disk_map(read_input(input));

  let mut defragmentator = Defragmentator {
    expanded_disk_map: expand(disk_map),
    previous_free_block: 0
  };

  defragmentator.defragment();
  defragmentator.checksum()
}

fn parse_disk_map(lines: Vec<String>) -> Vec<u32> {
  lines
    .first()
    .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
    .unwrap()
}

struct Defragmentator {
  expanded_disk_map: Vec<i32>,
  previous_free_block: usize
}

impl Defragmentator {
  fn defragment(&mut self) {
    let file_block_positions = self.expanded_disk_map.iter().enumerate()
      .filter(|x| *x.1 != FREE)
      .map(|x| x.0)
      .rev()
      .collect_vec();

    for file_pos in file_block_positions {
      if let Some(free_pos) = self.find_next_free_block().filter(|&p| p < file_pos) {
        self.expanded_disk_map.swap(file_pos, free_pos);
      }
    }
  }
  
  fn find_next_free_block(&self) -> Option<usize> {
    self.expanded_disk_map.iter()
      .skip(self.previous_free_block)
      .find_position(|d| **d == FREE)
      .map(|p|p.0)
  }

  fn checksum(&self) -> u64 {
    self.expanded_disk_map.iter().enumerate()
      .filter(|(_, d)| **d != FREE)
      .map(|(i, d)| i as u64 * *d as u64)
      .sum()
  }
}

fn expand(disk_map: Vec<u32>) -> Vec<i32> {
  disk_map
    .iter()
    .enumerate()
    .flat_map(|(i, &d)| {
      if i % 2 == 0 {
        (0..d).map(|_| (i / 2) as i32).collect_vec()
      } else {
        (0..d).map(|_| FREE).collect_vec()
      }
    })
    .collect_vec()
}

#[cfg(test)]
mod tests {
  use crate::{day09::checksum, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(checksum(&mut read("./src/day09/sample.input")), 1928)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(checksum(&mut read("./src/day09/my.input")), 6356833654075)
  }
}
