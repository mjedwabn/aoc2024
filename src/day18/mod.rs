use std::{collections::{BinaryHeap, HashMap, HashSet}, io::BufRead};

use itertools::Itertools;

use crate::{read_input, CartesianGrid, Coords, GridCoords};

pub fn minimum_number_of_steps_needed_to_reach_the_exit(input: &mut dyn BufRead, size: (usize, usize), n: usize) -> usize {
  let bytes = parse_input(read_input(input));
  let mut space = CartesianGrid::empty(size);
  for b in bytes.iter().take(n) {
    space.set(&b, '#');
  }
  let start = Coords(0, 0);
  let end = Coords(size.0 - 1, size.1 - 1);

  find_shortest_path_length(&space, start, end).unwrap()
}

pub fn find_first_byte_that_will_prevent_the_exit(input: &mut dyn BufRead, size: (usize, usize)) -> String {
  let bytes = parse_input(read_input(input));
  
  for n in 1..=bytes.len() {
    let mut space = CartesianGrid::empty(size);
    for b in bytes.iter().take(n) {
      space.set(&b, '#');
    }

    if find_shortest_path_length(&space, Coords(0, 0), Coords(size.0 - 1, size.1 - 1)).is_none() {
      return bytes.get(n - 1).map(|c| format!("{},{}", c.0, c.1)).unwrap();
    }
  }
  
  panic!("No byte will prevent the exit");
}

fn find_shortest_path_length(space: &CartesianGrid<char>, start: Coords, end: Coords) -> Option<usize> {
  let mut visited: HashSet<Coords> = HashSet::new();
  let mut dist: HashMap<Coords, usize> = HashMap::new();
  let mut queue: BinaryHeap<State> = BinaryHeap::new();
  queue.push(State::new(start, 0));
  
  while let Some(s) = queue.pop() {
    if s.position == end {
      return Some(s.cost);
    }
    
    visited.insert(s.position);
    
    for neighbor in space.neighbors4(&s.position) {
      let old = dist.get(&neighbor);
      let alt = s.cost + 1;

      if !visited.contains(&neighbor) {
        if old.is_none() || alt < *old.unwrap() {
          dist.insert(neighbor, alt);
          queue.push(State::new(neighbor, alt));
        }
      }
    }
  }
  
  None
}

fn parse_input(lines: Vec<String>) -> Vec<Coords> {
  fn parse_coord(line: &str) -> Coords {
    line.split_once(',').map(|(x, y)| Coords(x.parse().unwrap(), y.parse().unwrap())).unwrap()
  }
  lines.iter().map(|line| parse_coord(line)).collect()
}

#[derive(Eq, PartialEq)]
struct State {
  position: Coords,
  cost: usize
}

impl State {
  fn new(position: Coords, cost: usize) -> State {
    State {
      position,
      cost
    }
  }
}

impl CartesianGrid<char> {
  fn neighbors4(&self, coords: &Coords) -> Vec<Coords> {
    vec![
      coords.sub_y(1),
      coords.add_x(1),
      coords.add_y(1),
      coords.sub_x(1)
    ]
    .iter()
    .filter(|c| c.in_grid(self))
    .flat_map(|c| c.to_coords())
    .filter(|c| *self.get(c) != '#')
    .collect_vec()
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other.cost.cmp(&self.cost)
  }
}

#[cfg(test)]
mod tests {
  use crate::{day18::{find_first_byte_that_will_prevent_the_exit, minimum_number_of_steps_needed_to_reach_the_exit}, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(minimum_number_of_steps_needed_to_reach_the_exit(&mut read("./src/day18/sample.input"), (7, 7), 12), 22)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(minimum_number_of_steps_needed_to_reach_the_exit(&mut read("./src/day18/my.input"), (71, 71), 1024), 374)
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(find_first_byte_that_will_prevent_the_exit(&mut read("./src/day18/sample.input"), (7, 7)), "6,1")
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(find_first_byte_that_will_prevent_the_exit(&mut read("./src/day18/my.input"), (71, 71)), "30,12")
  }
}