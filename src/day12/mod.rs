use std::{collections::HashSet, io::BufRead};

use itertools::Itertools;

use crate::{CartesianGrid, Coords, ICoords, read_input};

pub fn total_price(input: &mut dyn BufRead) -> usize {
  let garden = CartesianGrid::from(read_input(input));
  garden.total_price()
}

trait Garden {
  fn total_price(&self) -> usize;
  fn plot_perimeter(&self, coords: &Coords) -> PlotPerimeter;
  fn get_adjacent_plots(&self, plot: &Coords) -> Vec<Coords>;
  fn detect_regions(&self) -> Vec<Vec<Coords>>;
}

type PlotPerimeter = (char, usize);

impl Garden for CartesianGrid<char> {
  fn total_price(&self) -> usize {
    self
      .detect_regions()
      .iter()
      .map(|region| {
        region
          .iter()
          .map(|plot| self.plot_perimeter(plot))
          .collect_vec()
      })
      .map(|plots| plots.len() * plots.iter().map(|(_, perimeter)| perimeter).sum::<usize>())
      .sum()
  }

  fn detect_regions(&self) -> Vec<Vec<Coords>> {
    let mut visited: HashSet<Coords> = HashSet::new();
    let mut regions: Vec<Vec<Coords>> = Vec::new();

    for c in self.coords() {
      if !visited.contains(&c) {
        let mut to_visit: Vec<Coords> = Vec::new();
        let mut region: Vec<Coords> = Vec::new();
        to_visit.push(c);

        while !to_visit.is_empty() {
          let v = to_visit.remove(0);
          visited.insert(v);
          region.push(v);

          for p in self.get_adjacent_plots(&v) {
            if !visited.contains(&p) && !to_visit.contains(&p) {
              to_visit.push(p);
            }
          }
        }

        regions.push(region);
      }
    }

    regions
  }

  fn get_adjacent_plots(&self, plot: &Coords) -> Vec<Coords> {
    let plot_type = self.get(plot);

    self
      .get_adjacent_coords_in_bounds(*plot)
      .iter()
      .filter(|c| self.get(c) == plot_type)
      .map(|c| *c)
      .collect_vec()
  }

  fn plot_perimeter(&self, coords: &Coords) -> PlotPerimeter {
    let plot_type = self.get(coords);

    let perimeter = self
      .get_adjacent_coords(*coords)
      .iter()
      .filter(|c| !self.in_grid(c) || self.get(&(c.0 as usize, c.1 as usize)) != plot_type)
      .count();

    (*plot_type, perimeter)
  }
}

impl CartesianGrid<char> {
  fn get_adjacent_coords(&self, coords: Coords) -> Vec<ICoords> {
    vec![
      (coords.0 as isize, coords.1 as isize + 1),
      (coords.0 as isize + 1, coords.1 as isize),
      (coords.0 as isize, coords.1 as isize - 1),
      (coords.0 as isize - 1, coords.1 as isize),
    ]
  }

  fn get_adjacent_coords_in_bounds(&self, coords: Coords) -> Vec<Coords> {
    vec![
      (coords.0 as isize, coords.1 as isize + 1),
      (coords.0 as isize + 1, coords.1 as isize),
      (coords.0 as isize, coords.1 as isize - 1),
      (coords.0 as isize - 1, coords.1 as isize),
    ]
    .iter()
    .filter(|c| self.in_grid(c))
    .map(|c| (c.0 as usize, c.1 as usize))
    .collect_vec()
  }
}

#[cfg(test)]
mod tests {
  use crate::{day12::total_price, read};

  #[test]
  fn sample1_part1_input() {
    assert_eq!(total_price(&mut read("./src/day12/sample1.input")), 140)
  }

  #[test]
  fn sample2_part1_input() {
    assert_eq!(total_price(&mut read("./src/day12/sample2.input")), 772)
  }

  #[test]
  fn sample3_part1_input() {
    assert_eq!(total_price(&mut read("./src/day12/sample3.input")), 1930)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(total_price(&mut read("./src/day12/my.input")), 1449902)
  }
}
