use std::{collections::{BTreeSet, HashSet}, io::BufRead};

use crate::read_input;

pub fn count_possible_designs(input: &mut dyn BufRead) -> usize {
  let (patterns, designs) = parse_input(read_input(input));

  designs.iter().filter(|d| is_design_possible(d, &patterns)).count()
}

pub fn count_different_ways_to_make_designs(input: &mut dyn BufRead) -> usize {
  let (patterns, designs) = parse_input(read_input(input));
  designs.iter()
    .map(|d| count_ways_to_make_design(d, &patterns))
    .sum()
}

fn is_design_possible(design: &String, patterns: &HashSet<String>) -> bool {
  let mut queue: BTreeSet<usize> = BTreeSet::new();
  queue.insert(0);

  while let Some(position) = queue.pop_first() {
    for candidate in patterns.iter().filter(|p| design[position..].starts_with(*p)) {
      let next_position = position + candidate.len();
    
      if next_position == design.len() {
        return true;
      }
    
      queue.insert(next_position);
    }
  }
  
  false
}

fn count_ways_to_make_design(design: &String, patterns: &HashSet<String>) -> usize {
  let mut queue: BTreeSet<usize> = BTreeSet::new();
  let mut ways = vec![0; design.len() + 1];
  queue.insert(0);
  ways[0] = 1;

  while let Some(position) = queue.pop_first() {
    for candidate in patterns.iter().filter(|p| design[position..].starts_with(*p)) {
      let next_position = position + candidate.len();
      ways[next_position] += ways[position];
    
      queue.insert(next_position);
    }
  }

  ways[design.len()]
}

fn parse_input(lines: Vec<String>) -> (HashSet<String>, Vec<String>) {
  let mut parts = lines.split(|line| line.is_empty());

  let patterns = parts.next().unwrap().get(0).unwrap().split(',').map(|p| p.trim().to_string()).collect();
  let designs = parts.next().unwrap().iter().map(|d| d.to_string()).collect();

  (patterns, designs)
}

#[cfg(test)]
mod tests {
  use crate::{day19::{count_different_ways_to_make_designs, count_possible_designs}, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(count_possible_designs(&mut read("./src/day19/sample.input")), 6)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(count_possible_designs(&mut read("./src/day19/my.input")), 290)
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(count_different_ways_to_make_designs(&mut read("./src/day19/sample.input")), 16)
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(count_different_ways_to_make_designs(&mut read("./src/day19/my.input")), 712058625427487)
  }
}