use std::io::BufRead;

use itertools::Itertools;

use crate::read_input;

pub fn checksum(input: &mut dyn BufRead) -> u64 {
  let disk_map = parse_disk_map(read_input(input));

  let mut defragmentator = Defragmentator {
    expanded_disk_map: expand(disk_map),
    previous_free_block: 0
  };

  defragmentator.defragment();
  defragmentator.checksum()
}

pub fn checksum_v2(input: &mut dyn BufRead) -> u64 {
  let disk_map = parse_disk_map(read_input(input));

  let mut defragmentator = Defragmentator {
    expanded_disk_map: expand(disk_map),
    previous_free_block: 0
  };

  defragmentator.defragment_whole_files();
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
      .filter(|x| *x.1 >= 0)
      .map(|x| x.0)
      .rev()
      .collect_vec();

    for file_pos in file_block_positions {
      if let Some(free_pos) = self.find_next_free_block().filter(|&p| p < file_pos) {
        self.expanded_disk_map.swap(file_pos, free_pos);
      }
    }
  }

  fn defragment_whole_files(&mut self) {
    let grouped_file_blocks = self.expanded_disk_map.iter()
      .enumerate()
      .filter(|x| *x.1 >= 0)
      .into_group_map_by(|f| f.1)
      .into_iter()
      .map(|x| (*x.0, x.1[0].0, x.1.len()))
      .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
      .collect_vec();

    let mut grouped_free_blocks = self.expanded_disk_map.iter()
      .enumerate()
      .filter(|x| *x.1 < 0)
      .into_group_map_by(|f| f.1)
      .into_iter()
      .map(|x| (x.1[0].0, x.1.len()))
      .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
      .enumerate()
      .collect_vec();

    for file_block in grouped_file_blocks {
      if let Some(free_block) = self.find_free_block(&grouped_free_blocks, file_block.2).filter(|&p| p.1 < file_block.1) {
        for pos in (file_block.1..file_block.1+file_block.2).enumerate() {
          self.expanded_disk_map.swap(pos.1, free_block.1 + pos.0);

          grouped_free_blocks[free_block.0].1.1 -= 1;
          grouped_free_blocks[free_block.0].1.0 += 1;
        }
      }
    }
  }
  
  fn find_next_free_block(&self) -> Option<usize> {
    self.expanded_disk_map.iter()
      .skip(self.previous_free_block)
      .find_position(|d| **d < 0)
      .map(|p|p.0)
  }

  fn find_free_block(&self, blocks: &Vec<(usize, (usize, usize))>, size: usize) -> Option<(usize, usize, usize)> {
    blocks.iter().find(|b| b.1.1 >= size).map(|b| (b.0, b.1.0, b.1.1))
  }

  fn checksum(&self) -> u64 {
    self.expanded_disk_map.iter().enumerate()
      .filter(|(_, d)| **d >= 0)
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
        (0..d).map(|_| -(i as i32) / 2 - 1).collect_vec()
      }
    })
    .collect_vec()
}

#[cfg(test)]
mod tests {
  use crate::{day09::{checksum, checksum_v2}, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(checksum(&mut read("./src/day09/sample.input")), 1928)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(checksum(&mut read("./src/day09/my.input")), 6356833654075)
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(checksum_v2(&mut read("./src/day09/sample.input")), 2858)
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(checksum_v2(&mut read("./src/day09/my.input")), 6389911791746)
  }
}
