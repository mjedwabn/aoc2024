use std::io::BufRead;

use itertools::Itertools;

use crate::{read_input, CartesianGrid, Coords, ICoords};

pub fn sum_boxes_gps_coordinates(input: &mut dyn BufRead) -> usize {
  let lines = read_input(input);
  let mut parts = lines.split(|line| line == "");
  let mut warehouse = Warehouse::new(CartesianGrid::from(parts.next().unwrap().to_vec()));
  let moves = parse_moves(parts.next().unwrap());

  for m in &moves {
    warehouse.attempt_move(m);
  }

  warehouse.get_boxes().iter().map(|c| c.0 + c.1 * 100).sum()
}

pub fn sum_scaled_up_boxes_gps_coordinates(input: &mut dyn BufRead) -> usize {
  let lines = read_input(input);
  let mut parts = lines.split(|line| line == "");
  let mut warehouse =
    Warehouse::new(CartesianGrid::from(parts.next().unwrap().to_vec())).scale_up();
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
        '^' => Ok(UnitVector::new(0, -1)),
        '>' => Ok(UnitVector::new(1, 0)),
        'v' => Ok(UnitVector::new(0, 1)),
        '<' => Ok(UnitVector::new(-1, 0)),
        _ => Err("Invalid character"),
      })
    })
    .collect()
}

struct Warehouse {
  grid: CartesianGrid<char>,
  current_position: Coords,
}

struct ScaledWarehouse {
  grid: CartesianGrid<char>,
  current_position: Coords,
}

impl ScaledWarehouse {
  fn get_boxes(&self) -> Vec<Coords> {
    self
      .grid
      .coords()
      .iter()
      .filter(|c| *self.grid.get(c) == '[')
      .map(|c| *c)
      .collect()
  }

  fn find_free_space(&self, direction: &UnitVector) -> Option<Coords> {
    self
      .get_coords_in_direction(&direction)
      .find(|c| *self.grid.get(c) == '.')
  }

  fn get_coords_in_direction(&self, direction: &UnitVector) -> impl Iterator<Item = Coords> {
    (1..)
      .flat_map(move |i| (self.current_position + direction * i).to_coords())
      .take_while(|c| *self.grid.get(c) == '[' || *self.grid.get(c) == ']' || *self.grid.get(c) == '.')
  }

  fn get_big_box_coordinates(&self, coords: Coords) -> BigBox {
    if self.get(&coords) == '[' {
      (coords, coords.add_x(1).to_coords().unwrap())
    }
    else {
      (coords.sub_x(1).to_coords().unwrap(), coords)
    }
  }

  fn can_bigbox_be_pushed(&self, bigbox: BigBox, direction: &UnitVector) -> bool {
    if (bigbox.0 + direction).to_coords().filter(|c| self.get(c) == '.').is_some() 
      && (bigbox.1 + direction).to_coords().filter(|c| self.get(c) == '.').is_some() {
      true
    }
    else if (bigbox.0 + direction).to_coords().filter(|c| self.get(c) == '#').is_some()
      || (bigbox.1 + direction).to_coords().filter(|c| self.get(c) == '#').is_some() {
      false
    }
    else {
      let left = (bigbox.0 + direction).to_coords().unwrap();
      let right = (bigbox.1 + direction).to_coords().unwrap();

      if self.get(&left) == '[' {
        self.can_bigbox_be_pushed(self.get_big_box_coordinates(left), direction)
      }
      else if self.get(&left) == ']' && self.get(&right) == '[' {
        self.can_bigbox_be_pushed(self.get_big_box_coordinates(left), direction)
          && self.can_bigbox_be_pushed(self.get_big_box_coordinates(right), direction)
      }
      else if self.get(&left) == ']' {
        self.can_bigbox_be_pushed(self.get_big_box_coordinates(left), direction)
      }
      else {
        self.can_bigbox_be_pushed(self.get_big_box_coordinates(right), direction)
      }
    }
  }

  fn push_bigbox(&mut self, bigbox: BigBox, direction: &UnitVector) {
    let mut boxes: Vec<BigBox> = Vec::new();
    let mut to_visit: Vec<Coords> = Vec::new();
    let left = (bigbox.0 + direction).to_coords().unwrap();
    let right = (bigbox.1 + direction).to_coords().unwrap();
    to_visit.push(left);
    to_visit.push(right);
    boxes.push(bigbox);

    while let Some(c) = to_visit.pop() {
      if self.get(&c) != '.' {
        let bb = self.get_big_box_coordinates(c);
        to_visit.push((bb.0 + direction).to_coords().unwrap());
        to_visit.push((bb.1 + direction).to_coords().unwrap());
        boxes.push(bb);
      }
    }

    for b in boxes.iter().unique().rev() {
      self.grid.set(&(b.0 + direction).to_coords().unwrap(), '[');
      self.grid.set(&(b.1 + direction).to_coords().unwrap(), ']');
      self.grid.set(&b.0, '.');
      self.grid.set(&b.1, '.');
    }
  }
}

trait Robot {
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
    self.get_field_in_direction(direction) != '#'
  }

  fn can_move(&self, direction: &UnitVector) -> bool {
    self.get_field_in_direction(direction) == '.'
  }

  fn get_field_in_direction(&self, direction: &UnitVector) -> char {
    self.get(&(self.current_position() + direction).to_coords().unwrap())
  }

  fn make_move(&mut self, direction: &UnitVector) {
    self.set(&self.current_position(), '.');
    let next_position = (self.current_position() + direction).to_coords().unwrap();
    self.set(&next_position, '@');
    self.set_current_position(next_position);
  }

  fn current_position(&self) -> Coords;
  fn set_current_position(&mut self, coords: Coords);
  fn get(&self, coords: &Coords) -> char;
  fn set(&mut self, coords: &Coords, value: char);
  fn try_push(&mut self, direction: &UnitVector);
}

impl Warehouse {
  fn new(grid: CartesianGrid<char>) -> Self {
    let starting_position = grid.find_one_coords('@').unwrap();

    Warehouse {
      grid: grid,
      current_position: starting_position,
    }
  }

  fn scale_up(&self) -> ScaledWarehouse {
    let scaled_up_grid: CartesianGrid<char> = CartesianGrid {
      grid: self
        .grid
        .grid
        .iter()
        .map(|row| {
          row
            .iter()
            .flat_map(|c| match c {
              '#' => vec!['#', '#'],
              'O' => vec!['[', ']'],
              '.' => vec!['.', '.'],
              '@' => vec!['@', '.'],
              _ => vec![],
            })
            .collect_vec()
        })
        .collect::<Vec<Vec<char>>>(),
    };

    let starting_position = scaled_up_grid.find_one_coords('@').unwrap();

    ScaledWarehouse {
      grid: scaled_up_grid,
      current_position: starting_position,
    }
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

  fn find_free_space(&self, direction: &UnitVector) -> Option<Coords> {
    self
      .get_coords_in_direction(&direction)
      .find(|c| *self.grid.get(c) == '.')
  }

  fn get_coords_in_direction(&self, direction: &UnitVector) -> impl Iterator<Item = Coords> {
    (1..)
    .flat_map(move |i| (self.current_position + direction * i).to_coords())
    .take_while(|c| *self.grid.get(c) == 'O' || *self.grid.get(c) == '.')
  }
}

impl Robot for Warehouse {
  fn current_position(&self) -> Coords {
    self.current_position
  }

  fn set_current_position(&mut self, coords: Coords) {
    self.current_position = coords
  }

  fn get(&self, coords: &Coords) -> char {
    *self.grid.get(coords)
  }

  fn set(&mut self, coords: &Coords, value: char) {
    self.grid.set(coords, value)
  }

  fn try_push(&mut self, direction: &UnitVector) {
    if let Some(free_space) = self.find_free_space(direction) {
      self.grid.set(&free_space, 'O');
      self.make_move(direction);
    }
  }
}

impl Robot for ScaledWarehouse {
  fn current_position(&self) -> Coords {
    self.current_position
  }

  fn set_current_position(&mut self, coords: Coords) {
    self.current_position = coords
  }

  fn get(&self, coords: &Coords) -> char {
    *self.grid.get(coords)
  }

  fn set(&mut self, coords: &Coords, value: char) {
    self.grid.set(coords, value)
  }

  fn try_push(&mut self, direction: &UnitVector) {
    if direction.1 == 0 {
      if let Some(free_space) = self.find_free_space(direction) {
        for w in self.grid.get_coords_between(&free_space, &self.current_position).windows(2) {
          self.set(&w[0], self.get(&w[1]));
        }
        self.make_move(direction)
      }
    }
    else {
      let bigbox = self.get_big_box_coordinates((self.current_position + direction).to_coords().unwrap());
      if self.can_bigbox_be_pushed(bigbox, direction) {
        self.push_bigbox(bigbox, direction);
        self.make_move(direction);
      }
    }
  }
}

type UnitVector = ICoords;
type BigBox = (Coords, Coords);

#[cfg(test)]
mod tests {
  use crate::{
    day15::{sum_boxes_gps_coordinates, sum_scaled_up_boxes_gps_coordinates},
    read,
  };

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

  #[test]
  fn smaller_sample_part2_input() {
    assert_eq!(
      sum_scaled_up_boxes_gps_coordinates(&mut read("./src/day15/smaller.part2.sample.input")),
      105 + 2 * 100 + 7 + 3 * 100 + 6
    )
  }

  #[test]
  fn larger_sample_part2_input() {
    assert_eq!(
      sum_scaled_up_boxes_gps_coordinates(&mut read("./src/day15/larger.sample.input")),
      9021
    )
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(
      sum_scaled_up_boxes_gps_coordinates(&mut read("./src/day15/my.input")),
      1467145
    )
  }
}
