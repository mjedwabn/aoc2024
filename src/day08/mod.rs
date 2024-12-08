use std::io::BufRead;

use itertools::Itertools;

use crate::{read_input, CartesianGrid, Coords};

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
    let dx = antenna1.0 as isize - antenna2.0 as isize;
    let dy = antenna1.1 as isize - antenna2.1 as isize;

    let antenna1_antinodes = harmonics().into_iter()
      .map(|i| i as isize)
      .map(|i| (antenna1.0 as isize + dx * i, antenna1.1 as isize + dy * i))
      .take_while(|c| self.in_grid(c));
    let antenna2_antinodes = harmonics().into_iter()
      .map(|i| i as isize)
      .map(|i| (antenna2.0 as isize - dx * i, antenna2.1 as isize - dy * i))
      .take_while(|c| self.in_grid(c));

    antenna1_antinodes.chain(antenna2_antinodes)
      .map(|c| (c.0 as usize, c.1 as usize))
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
