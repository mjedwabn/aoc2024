use std::io::BufRead;

use itertools::Itertools;

use crate::{read_input, CartesianGrid};

pub fn count_xmas_word(input: &mut dyn BufRead) -> usize {
  let word = "XMAS".chars().collect_vec();

  let board = parse_board(read_input(input));
  board.grid
    .coords()
    .iter()
    .map(|c| board.count_word(c, &word))
    .sum()
}

pub fn count_x_mas(input: &mut dyn BufRead) -> usize {
  let word = "MAS".chars().collect_vec();

  let board = parse_board(read_input(input));
  board.grid
    .coords()
    .iter()
    .map(|c| board.find_word_on_diagonals(c, &word))
    .flat_map(|words| {
      words
        .iter()
        .map(|w| *w.get(1).unwrap())
        .collect::<Vec<(usize, usize)>>()
    })
    .into_group_map_by(|&n| n)
    .into_iter()
    .filter(|(_, v)| v.len() == 2)
    .count()
}

fn parse_board(input: Vec<String>) -> Board {
  Board {
    grid: CartesianGrid {
      grid: input
        .iter()
        .map(|line| line.chars().into_iter().collect())
        .collect(),
    }
  }
}

struct Board {
  grid: CartesianGrid<char>
}

impl Board {
  fn count_word(&self, start: &(usize, usize), word: &Vec<char>) -> usize {
    if *self.grid.get(start) != *word.get(0).unwrap() {
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
    start: &(usize, usize),
    word: &Vec<char>,
  ) -> Vec<Vec<(usize, usize)>> {
    if *self.grid.get(start) != *word.get(0).unwrap() {
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
    start: &(usize, usize),
    direction: &(i8, i8),
    word: &Vec<char>,
  ) -> bool {
    if let Some(coords) = self.get_word_coords(start, direction, word.len()) {
      coords
        .iter()
        .enumerate()
        .all(|(i, c)| self.grid.get(c) == word.get(i).unwrap())
    } else {
      false
    }
  }

  fn get_word_coords(
    &self,
    start: &(usize, usize),
    direction: &(i8, i8),
    distance: usize,
  ) -> Option<Vec<(usize, usize)>> {
    if self.grid.in_grid(&(
      start.0 as isize + direction.0 as isize * (distance - 1) as isize,
      start.1 as isize + direction.1 as isize * (distance - 1) as isize,
    )) {
      Some(
        self.make_word_coords(start, direction, distance)
      )
    } else {
      None
    }
  }

  fn make_word_coords(
    &self,
    start: &(usize, usize),
    direction: &(i8, i8),
    distance: usize,
  ) -> Vec<(usize, usize)> {
    (0..distance)
      .map(|d| {
        (
          (start.0 as isize + direction.0 as isize * d as isize) as usize,
          (start.1 as isize + direction.1 as isize * d as isize) as usize,
        )
      })
      .collect::<Vec<(usize, usize)>>()
  }

  fn get_versors(&self) -> Vec<(i8, i8)> {
    return vec![
      (0, 1),
      (1, 1),
      (1, 0),
      (1, -1),
      (0, -1),
      (-1, -1),
      (-1, 0),
      (-1, 1),
    ];
  }

  fn get_diagonal_versors(&self) -> Vec<(i8, i8)> {
    return vec![(1, 1), (1, -1), (-1, -1), (-1, 1)];
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
