use std::{collections::{BTreeSet, HashSet}, io::BufRead};

use crate::read_input;

pub fn count_possible_designs(input: &mut dyn BufRead) -> usize {
  let (patterns, designs) = parse_input(read_input(input));

  designs.iter().filter(|d| is_design_possible(d, &patterns)).count()
}

fn is_design_possible(design: &String, patterns: &HashSet<String>) -> bool {
  let mut queue: BTreeSet<usize> = BTreeSet::new();
  queue.insert(0);

  while let Some(position) = queue.pop_first() {
    let candidates: Vec<&String> = patterns.iter().filter(|p| design[position..].starts_with(*p)).collect();
    
    for candidate in candidates {
      let next_position = position + candidate.len();
    
      if next_position == design.len() {
        return true;
      }
    
      queue.insert(next_position);
    }
  }
  
  false
}


fn parse_input(lines: Vec<String>) -> (HashSet<String>, Vec<String>) {
  let mut parts = lines.split(|line| line.is_empty());

  let patterns = parts.next().unwrap().get(0).unwrap().split(',').map(|p| p.trim().to_string()).collect();
  let designs = parts.next().unwrap().iter().map(|d| d.to_string()).collect();

  (patterns, designs)
}

#[cfg(test)]
mod tests {
  use crate::{day19::count_possible_designs, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(count_possible_designs(&mut read("./src/day19/sample.input")), 6)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(count_possible_designs(&mut read("./src/day19/my.input")), 290)
  }
}