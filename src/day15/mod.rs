use std::io::BufRead;

use crate::{CartesianGrid, Coords, read_input};

pub fn sum_boxes_gps_coordinates(input: &mut dyn BufRead) -> usize {
  let lines = read_input(input);
  let mut parts = lines.split(|line| line == "");
  let mut warehouse = Robot::new(CartesianGrid::from(parts.next().unwrap().to_vec()));
  let moves = parse_moves(parts.next().unwrap());

  for m in &moves {
    warehouse.attempt_move(m);
  }

  warehouse.get_boxes().iter().map(|c| c.0 + c.1 * 100).sum()
}

fn parse_moves(moves: &[String]) -> Vec<UnitVector> {
  moves
    .iter()
    .flat_map(|line| {
      line.chars().flat_map(|c| match c {
        '^' => Ok((0, -1)),
        '>' => Ok((1, 0)),
        'v' => Ok((0, 1)),
        '<' => Ok((-1, 0)),
        _ => Err("Invalid character"),
      })
    })
    .collect()
}

struct Robot {
  grid: CartesianGrid<char>,
  current_position: Coords,
}

impl Robot {
  fn new(grid: CartesianGrid<char>) -> Self {
    let starting_position = grid.find_one_coords('@').unwrap();

    Robot {
      grid: grid,
      current_position: starting_position,
    }
  }

  fn attempt_move(&mut self, direction: &UnitVector) {
    if self.can_make_any_move(direction) {
      if self.can_move(direction) {
        self.make_move(direction)
      } else {
        self.try_push(direction);
      }
    }
  }

  fn can_make_any_move(&self, direction: &UnitVector) -> bool {
    *self
      .grid
      .get(&self.calc_position(&self.current_position, direction))
      != '#'
  }

  fn can_move(&self, direction: &UnitVector) -> bool {
    *self
      .grid
      .get(&self.calc_position(&self.current_position, direction))
      == '.'
  }

  fn try_push(&mut self, direction: &UnitVector) {
    if let Some(free_space) = self.find_free_space(direction) {
      self.grid.set(&free_space, 'O');
      self.make_move(direction);
    }
  }

  fn find_free_space(&self, direction: &UnitVector) -> Option<Coords> {
    self
      .get_coords_in_direction(&direction)
      .find(|c| *self.grid.get(c) == '.')
  }

  fn get_coords_in_direction(&self, direction: &UnitVector) -> impl Iterator<Item = Coords> {
    (1..)
      .map(|i| {
        (
          (self.current_position.0 as isize + direction.0 * i) as usize,
          (self.current_position.1 as isize + direction.1 * i) as usize,
        )
      })
      .take_while(|c| *self.grid.get(c) == 'O' || *self.grid.get(c) == '.')
  }

  fn calc_position(&self, from: &Coords, direction: &UnitVector) -> Coords {
    (
      (from.0 as isize + direction.0) as usize,
      (from.1 as isize + direction.1) as usize,
    )
  }

  fn make_move(&mut self, direction: &UnitVector) {
    self.grid.set(&self.current_position, '.');
    let next_position = self.calc_position(&self.current_position, &direction);
    self
      .grid
      .set(&(next_position.0 as usize, next_position.1 as usize), '@');
    self.current_position = (next_position.0 as usize, next_position.1 as usize);
  }

  fn get_boxes(&self) -> Vec<Coords> {
    self
      .grid
      .coords()
      .iter()
      .filter(|c| *self.grid.get(c) == 'O')
      .map(|c| *c)
      .collect()
  }
}

type UnitVector = (isize, isize);

#[cfg(test)]
mod tests {
  use crate::{day15::sum_boxes_gps_coordinates, read};

  #[test]
  fn smaller_sample_part1_input() {
    assert_eq!(
      sum_boxes_gps_coordinates(&mut read("./src/day15/smaller.sample.input")),
      2028
    )
  }

  #[test]
  fn bigger_sample_part1_input() {
    assert_eq!(
      sum_boxes_gps_coordinates(&mut read("./src/day15/larger.sample.input")),
      10092
    )
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(
      sum_boxes_gps_coordinates(&mut read("./src/day15/my.input")),
      1457740
    )
  }
}
