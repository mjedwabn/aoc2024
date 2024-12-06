use std::{collections::HashSet, io::BufRead};

use crate::read_input;

pub fn count_positions_visited_by_guard(input: &mut dyn BufRead) -> usize {
  let mut guard = parse_map(read_input(input));
  while !guard.left_mapped_area() {
    guard.make_move();
  }
  guard.visited_positions.len()
}

fn parse_map(lines: Vec<String>) -> Guard {
  let grid = lines
    .iter()
    .map(|line| line.chars().into_iter().collect())
    .collect::<Vec<Vec<char>>>();

  Guard::new(grid)
}

struct CartesianGrid {
  grid: Vec<Vec<char>>
}

struct Guard {
  grid: CartesianGrid,
  position: Coords,
  direction: Direction,
  visited_positions: HashSet<Coords>
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
  fn new(g: Vec<Vec<char>>) -> Guard {
    let grid = CartesianGrid {
      grid: g
    };
    let starting_position = grid.find_coords('^').unwrap();
    Guard {
      grid,
      position: starting_position,
      direction: (0, -1),
      visited_positions: HashSet::from([starting_position])
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
    self.visited_positions.insert(self.position);
  }

  fn turn_right(&mut self) {
    self.direction = (
      -self.direction.1,
      self.direction.0
    )
  }
}

#[cfg(test)]
mod tests {
  use std::{fs::File, io::BufReader};

  use crate::day06::count_positions_visited_by_guard;

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
}
