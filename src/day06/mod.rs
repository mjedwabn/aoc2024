use std::{collections::HashMap, io::BufRead};

use crate::read_input;

pub fn count_positions_visited_by_guard(input: &mut dyn BufRead) -> usize {
  let mut guard = Guard::new(parse_map(read_input(input)));
  while !guard.left_mapped_area() {
    guard.make_move()
  }
  guard.count_visited_positions()
}

pub fn count_possible_loop_obstructions(input: &mut dyn BufRead) -> usize {
  let source_map = parse_map(read_input(input));
  let candidates = source_map.coords().iter()
    .filter(|c| *source_map.get(c) == '.')
    .map(|c| *c)
    .collect::<Vec<Coords>>();
  let mut count = 0;
  for candidate in candidates {
    //println!("candidate {:?}", candidate);
    let mut candidate_map = source_map.clone();
    candidate_map.set(&candidate, '#');
    let mut guard = Guard::new(candidate_map);
    while !guard.left_mapped_area() && !guard.entered_into_loop() {
      guard.make_move()
    }

    if guard.entered_into_loop() {
      count += 1
    }
  }
  count
}

fn parse_map(lines: Vec<String>) -> CartesianGrid {
  let grid = lines
    .iter()
    .map(|line| line.chars().into_iter().collect())
    .collect::<Vec<Vec<char>>>();

  CartesianGrid {grid}
}

#[derive(Clone)]
struct CartesianGrid {
  grid: Vec<Vec<char>>
}

struct Guard {
  grid: CartesianGrid,
  position: Coords,
  direction: Direction,
  visited_positions: HashMap<Coords, HashMap<Direction, usize>>
}

type Coords = (usize, usize);
type Direction = (isize, isize);

impl CartesianGrid {
  fn coords(&self) -> Vec<Coords> {
    return (0..self.grid.len())
      .flat_map(|y| (0..self.grid.get(y).unwrap().len()).map(move |x| (x, y)))
      .collect();
  }

  fn get(&self, coord: &Coords) -> &char {
    return self.grid.get(coord.1).unwrap().get(coord.0).unwrap();
  }

  fn set(&mut self, coord: &Coords, value: char) {
    self.grid.get_mut(coord.1).unwrap()[coord.0] = value
  }

  fn find_coords(&self, value: char) -> Option<(usize, usize)> {
    self.coords().iter().find(|c| *self.get(c) == value).map(|c| *c)
  }

  fn is_boundary(&self, coord: &Coords) -> bool {
    return coord.1 == 0
      || coord.1 == self.grid.len() - 1
      || coord.0 == 0
      || coord.0 == self.grid.get(coord.1 as usize).unwrap().len() - 1;
  }
}

impl Guard {
  fn new(grid: CartesianGrid) -> Guard {
    let starting_position = grid.find_coords('^').unwrap();
    let starting_direction = (0, -1);
    Guard {
      grid,
      position: starting_position,
      direction: starting_direction,
      visited_positions: HashMap::from([(starting_position, HashMap::from([(starting_direction, 1)]))])
    }
  }

  fn left_mapped_area(&self) -> bool {
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
    return *self.grid.get(&self.get_move_projection()) != '#'
  }

  fn get_move_projection(&self) -> Coords {
    return (
      (self.position.0 as isize + self.direction.0) as usize,
      (self.position.1 as isize + self.direction.1) as usize
    )
  }

  fn move_forward(&mut self) {
    self.position = self.get_move_projection();

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
    self.direction = (
      -self.direction.1,
      self.direction.0
    )
  }

  fn entered_into_loop(&self) -> bool {
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
  use std::{fs::File, io::BufReader};

  use crate::day06::{count_positions_visited_by_guard, count_possible_loop_obstructions};

  #[test]
  fn sample_part1_input() {
    let mut f = BufReader::new(File::open("./src/day06/sample.input").unwrap());
    assert_eq!(count_positions_visited_by_guard(&mut f), 41)
  }

  #[test]
  fn my_part1_input() {
    let mut f = BufReader::new(File::open("./src/day06/my.input").unwrap());
    assert_eq!(count_positions_visited_by_guard(&mut f), 5444)
  }

  #[test]
  fn sample_part2_input() {
    let mut f = BufReader::new(File::open("./src/day06/sample.input").unwrap());
    assert_eq!(count_possible_loop_obstructions(&mut f), 6)
  }

  #[test]
  fn my_part2_input() {
    let mut f = BufReader::new(File::open("./src/day06/my.input").unwrap());
    assert_eq!(count_possible_loop_obstructions(&mut f), 1946)
  }
}
