use std::{collections::HashMap, io::BufRead};

use crate::{read_input, CartesianGrid, Coords, ICoords};

pub fn count_positions_visited_by_guard(input: &mut dyn BufRead) -> usize {
  let mut guard = Guard::new(CartesianGrid::from(read_input(input)));
  while !guard.has_left_mapped_area() {
    guard.make_move()
  }
  guard.count_visited_positions()
}

pub fn count_possible_loop_obstructions(input: &mut dyn BufRead) -> usize {
  println!("w000");
  let source_map = CartesianGrid::from(read_input(input));
  println!("wf0");
  let candidates = source_map.coords().iter()
    .filter(|c| *source_map.get(c) == '.')
    .map(|c| *c)
    .collect::<Vec<Coords>>();
  println!("wf1");
  let mut count = 0;
  for candidate in candidates {
    println!("candidate {:?}", candidate);
    let mut candidate_map = source_map.clone();
    candidate_map.set(&candidate, '#');
    let mut guard = Guard::new(candidate_map);
    while !guard.has_left_mapped_area() && !guard.has_entered_into_loop() {
      guard.make_move()
    }

    if guard.has_entered_into_loop() {
      count += 1
    }
  }
  count
}

struct Guard {
  grid: CartesianGrid<char>,
  position: Coords,
  direction: Direction,
  visited_positions: HashMap<Coords, HashMap<Direction, usize>>
}

type Direction = ICoords;

impl Guard {
  fn new(grid: CartesianGrid<char>) -> Guard {
    let starting_position = grid.find_one_coords('^').unwrap();
    let starting_direction = ICoords(0, -1);
    Guard {
      grid,
      position: starting_position,
      direction: starting_direction,
      visited_positions: HashMap::from([(starting_position, HashMap::from([(starting_direction, 1)]))])
    }
  }

  fn has_left_mapped_area(&self) -> bool {
    self.grid.is_boundary(&self.position)
  }

  fn make_move(&mut self) {
    if self.move_is_possible() {
      self.move_forward()
    }
    else {
      self.turn_right()
    }
  }

  fn move_is_possible(&self) -> bool {
    self.get_move_projection().filter(|p| *self.grid.get(p) != '#').is_some()
  }

  fn get_move_projection(&self) -> Option<Coords> {
    (self.position + self.direction).to_coords()
  }

  fn move_forward(&mut self) {
    self.position = self.get_move_projection().unwrap();

    self.mark_position_visited()
  }

  fn mark_position_visited(&mut self) {
    if !self.visited_positions.contains_key(&self.position) {
      self.visited_positions.insert(self.position, HashMap::new());
    }
    if !self.visited_positions.get(&self.position).unwrap().contains_key(&self.direction) {
      self.visited_positions.get_mut(&self.position).unwrap().insert(self.direction, 0);
    }
    let count = *self.visited_positions.get(&self.position).unwrap().get(&self.direction).unwrap();
    self.visited_positions.get_mut(&self.position).unwrap().insert(self.direction, count + 1);
  }

  fn turn_right(&mut self) {
    self.direction = ICoords(
      -self.direction.1,
      self.direction.0
    )
  }

  fn has_entered_into_loop(&self) -> bool {
    *self.visited_positions.get(&self.position)
      .and_then(|x| x.get(&self.direction))
      .unwrap_or(&0) >= 2
  }

  fn count_visited_positions(&self) -> usize {
    self.visited_positions.keys().len()
  }
}

#[cfg(test)]
mod tests {
  use crate::{day06::{count_positions_visited_by_guard, count_possible_loop_obstructions}, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(count_positions_visited_by_guard(&mut read("./src/day06/sample.input")), 41)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(count_positions_visited_by_guard(&mut read("./src/day06/my.input")), 5444)
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(count_possible_loop_obstructions(&mut read("./src/day06/sample.input")), 6)
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(count_possible_loop_obstructions(&mut read("./src/day06/my.input")), 1946)
  }
}
