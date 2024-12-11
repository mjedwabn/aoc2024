use std::{
  collections::{HashMap, HashSet},
  io::BufRead,
};

use crate::{CartesianGrid, Coords, read_input};

pub fn sum_trailheads_scores(input: &mut dyn BufRead) -> u32 {
  let map = Map {
    grid: CartesianGrid::parse(read_input(input)),
  };
  map.sum_trailheads_scores()
}

struct Map {
  grid: CartesianGrid<i32>,
}

impl Map {
  fn sum_trailheads_scores(&self) -> u32 {
    let mut graph: HashMap<Coords, HashSet<Coords>> = HashMap::new();
    let mut to_visit: Vec<Coords> = Vec::new();
    let mut paths: HashSet<(Coords, Coords)> = HashSet::new();

    for h in self.grid.find_trailheads() {
      to_visit.push(h);

      while let Some(c) = to_visit.pop() {
        if !graph.contains_key(&c) {
          graph.insert(c, HashSet::new());
        }

        for n in self
          .grid
          .get_neighbours(c)
          .iter()
          .filter(|n| *self.grid.get(n) == self.grid.get(&c) + 1)
        {
          graph.get_mut(&c).unwrap().insert(*n);
          to_visit.push(*n);

          if *self.grid.get(n) == 9 {
            paths.insert((h, *n));
          }
        }
      }
    }

    paths.len() as u32
  }
}

impl CartesianGrid<i32> {
  fn parse(lines: Vec<String>) -> Self {
    Self {
      grid: lines
        .iter()
        .map(|line| {
          line
            .chars()
            .into_iter()
            .map(|c| c.to_digit(10).map(|v| v as i32).unwrap_or(-1))
            .collect()
        })
        .collect::<Vec<Vec<i32>>>(),
    }
  }

  fn find_trailheads(&self) -> Vec<Coords> {
    self
      .coords()
      .iter()
      .filter(|c| *self.get(c) == 0)
      .map(|c| *c)
      .collect::<Vec<Coords>>()
  }

  fn get_neighbours(&self, c: Coords) -> Vec<Coords> {
    vec![
      (c.0 as isize, c.1 as isize + 1),
      (c.0 as isize + 1, c.1 as isize),
      (c.0 as isize, c.1 as isize - 1),
      (c.0 as isize - 1, c.1 as isize),
    ]
    .iter()
    .filter(|x| self.in_grid(x))
    .map(|x| (x.0 as usize, x.1 as usize))
    .collect()
  }
}

#[cfg(test)]
mod tests {
  use crate::{day10::sum_trailheads_scores, read};

  #[test]
  fn sample1_part1_input() {
    assert_eq!(
      sum_trailheads_scores(&mut read("./src/day10/sample1.input")),
      1
    )
  }

  #[test]
  fn sample2_part1_input() {
    assert_eq!(
      sum_trailheads_scores(&mut read("./src/day10/sample2.input")),
      2
    )
  }

  #[test]
  fn sample3_part1_input() {
    assert_eq!(
      sum_trailheads_scores(&mut read("./src/day10/sample3.input")),
      4
    )
  }

  #[test]
  fn sample4_part1_input() {
    assert_eq!(
      sum_trailheads_scores(&mut read("./src/day10/sample4.input")),
      3
    )
  }

  #[test]
  fn sample5_part1_input() {
    assert_eq!(
      sum_trailheads_scores(&mut read("./src/day10/sample5.input")),
      36
    )
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(
      sum_trailheads_scores(&mut read("./src/day10/my.input")),
      638
    )
  }
}
