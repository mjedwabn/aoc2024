use std::io::BufRead;

use itertools::Itertools;

use crate::read_input;

pub fn count_unique_antinode_locations(input: &mut dyn BufRead) -> usize {
  let map = parse_map(read_input(input));
  map.count_unique_antinode_locations()
}

fn parse_map(lines: Vec<String>) -> CartesianGrid {
  let grid = lines
    .iter()
    .map(|line| line.chars().into_iter().collect())
    .collect::<Vec<Vec<char>>>();

  CartesianGrid { grid }
}

struct CartesianGrid {
  grid: Vec<Vec<char>>,
}

impl CartesianGrid {
  fn coords(&self) -> Vec<Coords> {
    (0..self.grid.len())
      .flat_map(|y| (0..self.grid.get(y).unwrap().len()).map(move |x| (x, y)))
      .collect()
  }

  fn get(&self, coord: &Coords) -> &char {
    self.grid.get(coord.1).unwrap().get(coord.0).unwrap()
  }

  fn in_grid(&self, coord: &(isize, isize)) -> bool {
    coord.1 >= 0
      && coord.1 < self.grid.len() as isize
      && coord.0 >= 0
      && coord.0 < self.grid.get(coord.1 as usize).unwrap().len() as isize
  }

  fn set(&mut self, coord: &Coords, value: char) {
    self.grid.get_mut(coord.1).unwrap()[coord.0] = value
  }

  fn print(&self) {
    for level in self.grid.iter() {
      for c in level {
          print!("{} ", c);
      }
      println!();
    }
  }
}

type Coords = (usize, usize);

trait Map {
  fn count_unique_antinode_locations(&self) -> usize;

  fn detect_frequency_antinodes(&self, antennas: Vec<&Coords>) -> Vec<Coords>;

  fn detect_antinodes(&self, antenna1: &Coords, antenna2: &Coords) -> Vec<Coords>;
}

impl Map for CartesianGrid {
  fn count_unique_antinode_locations(&self) -> usize {
    let antinodes = self.coords().iter().filter(|c| *self.get(c) != '.')
    .into_group_map_by(|c| self.get(c))
    .into_iter()
    .flat_map(|(_, antennas)| self.detect_frequency_antinodes(antennas))
    .unique().collect::<Vec<Coords>>();

    // println!("{:?}", antinodes);
    // let mut g = CartesianGrid { grid: self.grid.clone() };

    // println!("before");
    // g.print();

    // for c in &antinodes {
    //   g.set(&c, '#');
    // }

    // println!("after");
    // g.print();

    antinodes.len()
  }

  fn detect_frequency_antinodes(&self, antennas: Vec<&Coords>) -> Vec<Coords> {
    antennas.iter().combinations(2)
      .map(|combination| (combination[0], combination[1]))
      .flat_map(|(&antenna1, &antenna2)| self.detect_antinodes(antenna1, antenna2))
      .collect()
  }

  fn detect_antinodes(&self, antenna1: &Coords, antenna2: &Coords) -> Vec<Coords> {
    let dx = antenna1.0 as isize - antenna2.0 as isize;
    let dy = antenna1.1 as isize - antenna2.1 as isize;

    let theoretical_antinodes = vec![
      (antenna1.0 as isize + dx, antenna1.1 as isize + dy),
      (antenna2.0 as isize - dx, antenna2.1 as isize - dy)
    ];

    theoretical_antinodes.iter()
      .filter(|c| self.in_grid(*c))
      .map(|c| (c.0 as usize, c.1 as usize))
      .collect_vec()
  }
}

#[cfg(test)]
mod tests {
  use std::{fs::File, io::BufReader};
  use crate::day08::count_unique_antinode_locations;

  #[test]
  fn sample_part1_input() {
    let mut f = BufReader::new(File::open("./src/day08/sample.input").unwrap());
    assert_eq!(count_unique_antinode_locations(&mut f), 14)
  }

  #[test]
  fn my_part1_input() {
    let mut f = BufReader::new(File::open("./src/day08/my.input").unwrap());
    assert_eq!(count_unique_antinode_locations(&mut f), 276)
  }
}