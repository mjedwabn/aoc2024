use std::{collections::HashSet, io::BufRead};

use itertools::Itertools;

use crate::{CartesianGrid, Coords, ICoords, read_input};

pub fn total_price(input: &mut dyn BufRead) -> usize {
  let garden = CartesianGrid::from(read_input(input));
  garden.total_price()
}

pub fn total_discount_price(input: &mut dyn BufRead) -> usize {
  let garden = CartesianGrid::from(read_input(input));
  garden.total_discount_price()
}

trait Garden {
  fn total_price(&self) -> usize;
  fn total_discount_price(&self) -> usize;
  fn area(&self, region: &Vec<Coords>) -> usize;
  fn plot_perimeter(&self, coords: &Coords) -> PlotPerimeter;
  fn number_of_sides(&self, region: &Vec<Coords>) -> usize;
  fn requires_left_fence(&self, plot: &Coords) -> bool;
  fn requires_right_fence(&self, plot: &Coords) -> bool;
  fn requires_top_fence(&self, plot: &Coords) -> bool;
  fn requires_bottom_fence(&self, plot: &Coords) -> bool;
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

  fn total_discount_price(&self) -> usize {
    self
      .detect_regions()
      .iter()
      .map(|region| self.area(region) * self.number_of_sides(region))
      .sum()
  }

  fn area(&self, region: &Vec<Coords>) -> usize {
      region.len()
  }

  fn number_of_sides(&self, region: &Vec<Coords>) -> usize {
    let left_fences = region
      .iter()
      .filter(|p| self.requires_left_fence(p))
      .into_group_map_by(|p| p.0)
      .into_iter()
      .map(|g| g.1.iter().map(|c| c.1).sorted().collect_vec().windows(2).filter(|w| w[1] > w[0] + 1).count() + 1)
      .sum::<usize>();
    let right_fences = region
      .iter()
      .filter(|p| self.requires_right_fence(p))
      .into_group_map_by(|p| p.0)
      .into_iter()
      .map(|g| g.1.iter().map(|c| c.1).sorted().collect_vec().windows(2).filter(|w| w[1] > w[0] + 1).count() + 1)
      .sum::<usize>();
    let top_fences = region
      .iter()
      .filter(|p| self.requires_top_fence(p))
      .into_group_map_by(|p| p.1)
      .into_iter()
      .map(|g| g.1.iter().map(|c| c.0).sorted().collect_vec().windows(2).filter(|w| w[1] > w[0] + 1).count() + 1)
      .sum::<usize>();
    let bottom_fences = region
      .iter()
      .filter(|p| self.requires_bottom_fence(p))
      .into_group_map_by(|p| p.1)
      .into_iter()
      .map(|g| g.1.iter().map(|c| c.0).sorted().collect_vec().windows(2).filter(|w| w[1] > w[0] + 1).count() + 1)
      .sum::<usize>();

    left_fences + right_fences + top_fences + bottom_fences
  }

  fn requires_left_fence(&self, plot: &Coords) -> bool {
    let plot_type = self.get(plot);
    let c = plot.sub_x(1);
    !self.in_grid(&c) || self.get(&c.to_coords().unwrap()) != plot_type
  }

  fn requires_right_fence(&self, plot: &Coords) -> bool {
    let plot_type = self.get(plot);
    let c = plot.add_x(1);
    !self.in_grid(&c) || self.get(&c.to_coords().unwrap()) != plot_type
  }

  fn requires_top_fence(&self, plot: &Coords) -> bool {
    let plot_type = self.get(plot);
    let c = plot.sub_y(1);
    !self.in_grid(&c) || self.get(&c.to_coords().unwrap()) != plot_type
  }

  fn requires_bottom_fence(&self, plot: &Coords) -> bool {
    let plot_type = self.get(plot);
    let c = plot.add_y(1);
    !self.in_grid(&c) || self.get(&c.to_coords().unwrap()) != plot_type
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
      .filter(|c| !self.in_grid(c) || self.get(&c.to_coords().unwrap()) != plot_type)
      .count();

    (*plot_type, perimeter)
  }
}

impl CartesianGrid<char> {
  fn get_adjacent_coords(&self, coords: Coords) -> Vec<ICoords> {
    vec![
      coords.add_y(1),
      coords.add_x(1),
      coords.sub_y(1),
      coords.sub_x(1),
    ]
  }

  fn get_adjacent_coords_in_bounds(&self, coords: Coords) -> Vec<Coords> {
    vec![
      coords.add_y(1),
      coords.add_x(1),
      coords.sub_y(1),
      coords.sub_x(1),
    ]
    .iter()
    .filter(|c| self.in_grid(c))
    .map(|c| c.to_coords().unwrap())
    .collect_vec()
  }
}

#[cfg(test)]
mod tests {
  use crate::{day12::{total_discount_price, total_price}, read};

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

  #[test]
  fn sample1_part2_input() {
    assert_eq!(total_discount_price(&mut read("./src/day12/sample1.input")), 80)
  }

  #[test]
  fn sample2_part2_input() {
    assert_eq!(total_discount_price(&mut read("./src/day12/sample2.input")), 436)
  }

  #[test]
  fn sample4_part2_input() {
    assert_eq!(total_discount_price(&mut read("./src/day12/sample4.part2.input")), 236)
  }

  #[test]
  fn sample5_part2_input() {
    assert_eq!(total_discount_price(&mut read("./src/day12/sample5.part2.input")), 368)
  }

  #[test]
  fn sample3_part2_input() {
    assert_eq!(total_discount_price(&mut read("./src/day12/sample3.input")), 1206)
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(total_discount_price(&mut read("./src/day12/my.input")), 908042)
  }
}
