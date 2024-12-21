use std::io::BufRead;

use itertools::Itertools;

use crate::{read_input, CartesianGrid, Coords, GridCoords};

pub fn count_unique_antinode_locations(input: &mut dyn BufRead) -> usize {
  let map = CartesianGrid::from(read_input(input));
  map.count_unique_antinode_locations(|| (1..=1))
}

pub fn count_unique_harmonic_antinode_locations(input: &mut dyn BufRead) -> usize {
  let map = CartesianGrid::from(read_input(input));
  map.count_unique_antinode_locations(|| (0..))
}

trait Map {
  fn count_unique_antinode_locations<R>(&self, harmonics: fn() -> R) -> usize where R: IntoIterator<Item = u32>;
  fn detect_frequency_antinodes<R>(&self, antennas: Vec<&Coords>, harmonics: fn() -> R) -> Vec<Coords> where R: IntoIterator<Item = u32>;
  fn detect_antinodes<R>(&self, antenna1: &Coords, antenna2: &Coords, harmonics: fn() -> R) -> Vec<Coords> where R: IntoIterator<Item = u32>;
}

impl Map for CartesianGrid<char> {
  fn count_unique_antinode_locations<R>(&self, harmonics: fn() -> R) -> usize where R: IntoIterator<Item = u32> {
    println!("kek1");
    self.coords().iter().filter(|c| *self.get(c) != '.')
      .into_group_map_by(|c| self.get(c))
      .into_iter()
      .flat_map(|(_, antennas)| self.detect_frequency_antinodes(antennas, harmonics))
      .unique().collect::<Vec<Coords>>()
      .len()
  }

  fn detect_frequency_antinodes<R>(&self, antennas: Vec<&Coords>, harmonics: fn() -> R) -> Vec<Coords> where R: IntoIterator<Item = u32> {
    antennas.iter().combinations(2)
      .map(|combination| (combination[0], combination[1]))
      .flat_map(|(&antenna1, &antenna2)| self.detect_antinodes(antenna1, antenna2, harmonics))
      .collect()
  }

  fn detect_antinodes<R>(&self, antenna1: &Coords, antenna2: &Coords, harmonics: fn() -> R) -> Vec<Coords> where R: IntoIterator<Item = u32> {
    let d = antenna1 - antenna2;
    let antenna1_antinodes = harmonics().into_iter()
      .map(|i| i as isize)
      .map(|i| antenna1 + d * i)
      .take_while(|c| c.in_grid(self))
      .flat_map(|c| c.to_coords());
    let antenna2_antinodes = harmonics().into_iter()
      .map(|i| i as isize)
      .map(|i| antenna2 - d * i)
      .take_while(|c| c.in_grid(self))
      .flat_map(|c| c.to_coords());

    antenna1_antinodes.chain(antenna2_antinodes)
      .collect_vec()
  }
}

#[cfg(test)]
mod tests {
  use crate::{day08::{count_unique_antinode_locations, count_unique_harmonic_antinode_locations}, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(count_unique_antinode_locations(&mut read("./src/day08/sample.input")), 14)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(count_unique_antinode_locations(&mut read("./src/day08/my.input")), 276)
  }

  #[test]
  fn simple_sample_part2_input() {
    assert_eq!(count_unique_harmonic_antinode_locations(&mut read("./src/day08/simple_sample.input")), 9)
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(count_unique_harmonic_antinode_locations(&mut read("./src/day08/sample.input")), 34)
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(count_unique_harmonic_antinode_locations(&mut read("./src/day08/my.input")), 991)
  }
}
