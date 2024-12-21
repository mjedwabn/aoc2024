use std::io::BufRead;

use itertools::Itertools;

use crate::{read_input, CartesianGrid, Coords, GridCoords, ICoords};

pub fn count_xmas_word(input: &mut dyn BufRead) -> usize {
  let word = "XMAS".chars().collect_vec();

  let board = CartesianGrid::from(read_input(input));
  board
    .coords()
    .iter()
    .map(|c| board.count_word(c, &word))
    .sum()
}

pub fn count_x_mas(input: &mut dyn BufRead) -> usize {
  let word = "MAS".chars().collect_vec();

  let board = CartesianGrid::from(read_input(input));
  board
    .coords()
    .iter()
    .map(|c| board.find_word_on_diagonals(c, &word))
    .flat_map(|words| {
      words
        .iter()
        .map(|w| *w.get(1).unwrap())
        .collect::<Vec<Coords>>()
    })
    .into_group_map_by(|&n| n)
    .into_iter()
    .filter(|(_, v)| v.len() == 2)
    .count()
}

impl CartesianGrid<char> {
  fn count_word(&self, start: &Coords, word: &Vec<char>) -> usize {
    if *self.get(start) != *word.get(0).unwrap() {
      return 0;
    } else {
      self
        .get_versors()
        .iter()
        .filter(|v| self.word_exists_in_direction(start, v, word))
        .count()
    }
  }

  fn find_word_on_diagonals(
    &self,
    start: &Coords,
    word: &Vec<char>,
  ) -> Vec<Vec<Coords>> {
    if *self.get(start) != *word.get(0).unwrap() {
      return vec![];
    } else {
      self
        .get_diagonal_versors()
        .iter()
        .filter(|v| self.word_exists_in_direction(start, v, word))
        .map(|v| self.make_word_coords(start, v, word.len()))
        .collect()
    }
  }

  fn word_exists_in_direction(
    &self,
    start: &Coords,
    direction: &ICoords,
    word: &Vec<char>,
  ) -> bool {
    if let Some(coords) = self.get_word_coords(start, direction, word.len()) {
      coords
        .iter()
        .enumerate()
        .all(|(i, c)| self.get(c) == word.get(i).unwrap())
    } else {
      false
    }
  }

  fn get_word_coords(
    &self,
    start: &Coords,
    direction: &ICoords,
    distance: usize,
  ) -> Option<Vec<Coords>> {
    if (start + direction * (distance as i32 - 1)).in_grid(self) {
      Some(
        self.make_word_coords(start, direction, distance)
      )
    } else {
      None
    }
  }

  fn make_word_coords(
    &self,
    start: &Coords,
    direction: &ICoords,
    distance: usize,
  ) -> Vec<Coords> {
    (0..distance)
      .flat_map(|d| (start + direction * d as i32).to_coords())
      .collect::<Vec<Coords>>()
  }

  fn get_versors(&self) -> Vec<ICoords> {
    return vec![
      ICoords(0, 1),
      ICoords(1, 1),
      ICoords(1, 0),
      ICoords(1, -1),
      ICoords(0, -1),
      ICoords(-1, -1),
      ICoords(-1, 0),
      ICoords(-1, 1),
    ];
  }

  fn get_diagonal_versors(&self) -> Vec<ICoords> {
    return vec![ICoords(1, 1), ICoords(1, -1), ICoords(-1, -1), ICoords(-1, 1)];
  }
}

#[cfg(test)]
mod tests {
  use crate::{day04::{count_x_mas, count_xmas_word}, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(count_xmas_word(&mut read("./src/day04/sample.input")), 18);
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(count_xmas_word(&mut read("./src/day04/my.input")), 2406);
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(count_x_mas(&mut read("./src/day04/sample.input")), 9);
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(count_x_mas(&mut read("./src/day04/my.input")), 1807);
  }
}
