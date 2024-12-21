use std::io::BufRead;

use itertools::Itertools;

use crate::{CartesianGrid, Coords, read_input};

pub fn sum_trailheads_scores(input: &mut dyn BufRead) -> usize {
  let map = CartesianGrid::parse(read_input(input));
  map.sum_trailheads_scores()
}

pub fn sum_trailheads_ratings(input: &mut dyn BufRead) -> usize {
  let map = CartesianGrid::parse(read_input(input));
  map.sum_trailheads_ratings()
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

  fn sum_trailheads_scores(&self) -> usize {
    self.find_hiking_paths().iter().unique().count()
  }

  fn sum_trailheads_ratings(&self) -> usize {
    self.find_hiking_paths().len()
  }

  fn find_hiking_paths(&self) -> Vec<(Coords, Coords)> {
    let mut to_visit: Vec<Coords> = Vec::new();
    let mut paths: Vec<(Coords, Coords)> = Vec::new();

    for h in self.find_trailheads() {
      to_visit.push(h);

      while let Some(c) = to_visit.pop() {
        for n in self
          .get_neighbours(c)
          .iter()
          .filter(|n| *self.get(n) == self.get(&c) + 1)
        {
          to_visit.push(*n);

          if *self.get(n) == 9 {
            paths.push((h, *n));
          }
        }
      }
    }

    paths
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
      c.add_y(1),
      c.add_x(1),
      c.sub_y(1),
      c.sub_x(1),
    ]
    .iter()
    .filter(|x| self.in_grid(x))
    .map(|x| x.to_coords().unwrap())
    .collect()
  }
}

#[cfg(test)]
mod tests {
  use crate::{day10::{sum_trailheads_ratings, sum_trailheads_scores}, read};

  #[test]
  fn sample1_part1_input() {
    assert_eq!(
      sum_trailheads_scores(&mut read("./src/day10/sample1.part1.input")),
      1
    )
  }

  #[test]
  fn sample2_part1_input() {
    assert_eq!(
      sum_trailheads_scores(&mut read("./src/day10/sample2.part1.input")),
      2
    )
  }

  #[test]
  fn sample3_part1_input() {
    assert_eq!(
      sum_trailheads_scores(&mut read("./src/day10/sample3.part1.input")),
      4
    )
  }

  #[test]
  fn sample4_part1_input() {
    assert_eq!(
      sum_trailheads_scores(&mut read("./src/day10/sample4.part1.input")),
      3
    )
  }

  #[test]
  fn sample5_part1_input() {
    assert_eq!(
      sum_trailheads_scores(&mut read("./src/day10/sample.input")),
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

  #[test]
  fn sample1_part2_input() {
    assert_eq!(
      sum_trailheads_ratings(&mut read("./src/day10/sample1.part2.input")),
      3
    )
  }

  #[test]
  fn sample2_part2_input() {
    assert_eq!(
      sum_trailheads_ratings(&mut read("./src/day10/sample2.part2.input")),
      13
    )
  }

  #[test]
  fn sample3_part2_input() {
    assert_eq!(
      sum_trailheads_ratings(&mut read("./src/day10/sample3.part2.input")),
      227
    )
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(
      sum_trailheads_ratings(&mut read("./src/day10/sample.input")),
      81
    )
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(
      sum_trailheads_ratings(&mut read("./src/day10/my.input")),
      1289
    )
  }
}
