use std::{
  fs::File,
  io::{BufRead, BufReader},
  ops::{self},
};

use itertools::Itertools;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;

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

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Coords(usize, usize);

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct ICoords(isize, isize);

pub trait GridCoords {
  fn in_grid<T>(&self, grid: &CartesianGrid<T>) -> bool;
}

impl Coords {
  pub fn new(x: usize, y: usize) -> Self {
    Self { 0: x, 1: y }
  }
  
  fn sub_x(&self, v: i32) -> ICoords {
    ICoords::new(self.0 as isize - v as isize, self.1 as isize)
  }

  fn add_x(&self, v: usize) -> ICoords {
    ICoords::new(self.0 as isize + v as isize, self.1 as isize)
  }

  fn sub_y(&self, v: i32) -> ICoords {
    ICoords::new(self.0 as isize as isize, self.1 as isize - v as isize)
  }

  fn add_y(&self, v: usize) -> ICoords {
    ICoords::new(self.0 as isize as isize, self.1 as isize + v as isize)
  }
}

impl GridCoords for Coords {
  fn in_grid<T>(&self, grid: &CartesianGrid<T>) -> bool {
    self.1 < grid.grid.len()
      && self.0 < grid.grid.get(self.1).unwrap().len()
  }
}

impl GridCoords for ICoords {
  fn in_grid<T>(&self, grid: &CartesianGrid<T>) -> bool {
    self.1 >= 0
      && self.1 < grid.grid.len() as isize
      && self.0 >= 0
      && self.0 < grid.grid.get(self.1 as usize).unwrap().len() as isize
  }
}

impl ICoords {
  pub fn new(x: isize, y: isize) -> Self {
    Self { 0: x, 1: y }
  }

  pub fn rem_euclid(&self, x: usize, y: usize) -> Coords {
    Coords::new(self.0.rem_euclid(x as isize) as usize, self.1.rem_euclid(y as isize) as usize)
  }
  
  fn to_coords(&self) -> Option<Coords> {
    if self.0 >= 0 && self.1 >= 0 {
      Some(Coords::new(self.0 as usize, self.1 as usize))
    }
    else {
      None
    }
  }
}

impl ops::Add<Coords> for Coords {
  type Output = Coords;

  fn add(self, rhs: Coords) -> Self::Output {
    Self::Output {
      0: self.0 + rhs.0,
      1: self.1 + rhs.1,
    }
  }
}

impl ops::Mul<i32> for Coords {
  type Output = Coords;

  fn mul(self, n: i32) -> Self::Output {
    Self::Output {
      0: self.0 * n as usize,
      1: self.1 * n as usize,
    }
  }
}

impl ops::Add<ICoords> for Coords {
  type Output = ICoords;

  fn add(self, rhs: ICoords) -> Self::Output {
    ICoords {
      0: self.0 as isize + rhs.0,
      1: self.1 as isize + rhs.1,
    }
  }
}

impl ops::Add<&ICoords> for Coords {
  type Output = ICoords;

  fn add(self, rhs: &ICoords) -> Self::Output {
    ICoords {
      0: self.0 as isize + rhs.0,
      1: self.1 as isize + rhs.1,
    }
  }
}

impl ops::Add<&ICoords> for &Coords {
  type Output = ICoords;

  fn add(self, rhs: &ICoords) -> Self::Output {
    ICoords {
      0: self.0 as isize + rhs.0,
      1: self.1 as isize + rhs.1,
    }
  }
}

impl ops::Add<ICoords> for &Coords {
  type Output = ICoords;

  fn add(self, rhs: ICoords) -> Self::Output {
    ICoords {
      0: self.0 as isize + rhs.0,
      1: self.1 as isize + rhs.1,
    }
  }
}

impl ops::Sub<ICoords> for &Coords {
  type Output = ICoords;

  fn sub(self, rhs: ICoords) -> Self::Output {
    ICoords {
      0: self.0 as isize - rhs.0,
      1: self.1 as isize - rhs.1,
    }
  }
}

impl ops::Sub<&Coords> for &Coords {
  type Output = ICoords;

  fn sub(self, rhs: &Coords) -> Self::Output {
    Self::Output {
      0: self.0 as isize - rhs.0 as isize,
      1: self.1 as isize - rhs.1 as isize,
    }
  }
}

impl ops::Mul<i32> for ICoords {
  type Output = ICoords;

  fn mul(self, n: i32) -> Self::Output {
    Self::Output {
      0: self.0 * n as isize,
      1: self.1 * n as isize,
    }
  }
}

impl ops::Mul<isize> for ICoords {
  type Output = ICoords;

  fn mul(self, n: isize) -> Self::Output {
    Self::Output {
      0: self.0 * n as isize,
      1: self.1 * n as isize,
    }
  }
}

impl ops::Mul<i32> for &ICoords {
  type Output = ICoords;

  fn mul(self, n: i32) -> Self::Output {
    Self::Output {
      0: self.0 * n as isize,
      1: self.1 * n as isize,
    }
  }
}

impl ops::Mul<usize> for ICoords {
  type Output = ICoords;

  fn mul(self, n: usize) -> Self::Output {
    Self::Output {
      0: self.0 * n as isize,
      1: self.1 * n as isize,
    }
  }
}

impl<T: std::fmt::Display + std::cmp::PartialEq> CartesianGrid<T> {
  fn coords(&self) -> Vec<Coords> {
    (0..self.grid.len())
      .flat_map(|y| (0..self.grid.get(y).unwrap().len()).map(move |x| Coords::new(x, y)))
      .collect()
  }

  fn get(&self, coord: &Coords) -> &T {
    self.grid.get(coord.1).unwrap().get(coord.0).unwrap()
  }

  fn in_grid(&self, coord: &ICoords) -> bool {
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
    self
      .coords()
      .iter()
      .find(|c| *self.get(c) == value)
      .map(|c| *c)
  }

  fn set(&mut self, coord: &Coords, value: T) {
    self.grid.get_mut(coord.1).unwrap()[coord.0] = value
  }

  fn get_coords_between(&self, from: &Coords, to: &Coords) -> Vec<Coords> {
    let dx = from.0 as isize - to.0 as isize;
    let dy = from.1 as isize - to.1 as isize;

    if dx == 0 {
      if dy < 0 {
        (from.1..to.1).map(|y| Coords(from.1, y)).collect_vec()
      }
      else {
        (to.1 + 1..=from.1).map(|y| Coords(from.1, y)).rev().collect_vec()
      }
    }
    else if dy == 0 {
      if dx < 0 {
        (from.0..to.0).map(|x| Coords(x, from.1)).collect_vec()  
      }
      else {
        (to.0 + 1..=from.0).map(|x| Coords(x, from.1)).rev().collect_vec()
      }
    }
    else {
      // TODO: diagonals
      vec![]
    }
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
