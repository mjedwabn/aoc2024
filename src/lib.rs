use std::{fs::File, io::{BufRead, BufReader}};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

pub fn read_input(input: &mut dyn BufRead) -> Vec<String> {
  input
    .lines()
    .map(|line| line.unwrap())
    .collect::<Vec<String>>()
}

pub fn read(file_name: &str) -> BufReader<File> {
  BufReader::new(File::open(file_name).unwrap())
}

#[derive(Clone)]
pub struct CartesianGrid<T> {
  grid: Vec<Vec<T>>,
}

pub type Coords = (usize, usize);

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
    coord.1 == 0
      || coord.1 == self.grid.len() - 1
      || coord.0 == 0
      || coord.0 == self.grid.get(coord.1 as usize).unwrap().len() - 1
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

impl CartesianGrid<char> {
  pub fn from(lines: Vec<String>) -> Self {
    let grid = lines
      .iter()
      .map(|line| line.chars().into_iter().collect())
      .collect::<Vec<Vec<char>>>();
  
    CartesianGrid { grid }
  }
}