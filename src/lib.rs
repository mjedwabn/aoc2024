use std::io::BufRead;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

pub fn read_input(input: &mut dyn BufRead) -> Vec<String> {
  return input
    .lines()
    .map(|line| line.unwrap())
    .collect::<Vec<String>>();
}

#[derive(Clone)]
struct CartesianGrid<T> {
  grid: Vec<Vec<T>>,
}

type Coords = (usize, usize);

impl<T: std::fmt::Display + std::cmp::PartialEq> CartesianGrid<T> {
  fn coords(&self) -> Vec<Coords> {
    (0..self.grid.len())
      .flat_map(|y| (0..self.grid.get(y).unwrap().len()).map(move |x| (x, y)))
      .collect()
  }

  fn get(&self, coord: &Coords) -> &T {
    self.grid.get(coord.1).unwrap().get(coord.0).unwrap()
  }

  fn in_grid(&self, coord: &(isize, isize)) -> bool {
    coord.1 >= 0
      && coord.1 < self.grid.len() as isize
      && coord.0 >= 0
      && coord.0 < self.grid.get(coord.1 as usize).unwrap().len() as isize
  }

  fn is_boundary(&self, coord: &Coords) -> bool {
    return coord.1 == 0
      || coord.1 == self.grid.len() - 1
      || coord.0 == 0
      || coord.0 == self.grid.get(coord.1 as usize).unwrap().len() - 1;
  }

  fn find_one_coords(&self, value: T) -> Option<Coords> {
    self.coords().iter().find(|c| *self.get(c) == value).map(|c| *c)
  }

  fn set(&mut self, coord: &Coords, value: T) {
    self.grid.get_mut(coord.1).unwrap()[coord.0] = value
  }

  #[allow(dead_code)]
  fn print(&self) {
    for level in self.grid.iter() {
      for c in level {
        print!("{} ", c);
      }
      println!();
    }
  }
}
