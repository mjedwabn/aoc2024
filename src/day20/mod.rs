use std::{collections::{BinaryHeap, HashMap, HashSet}, io::BufRead};

use itertools::Itertools;

use crate::{read_input, CartesianGrid, Coords, GridCoords, ICoords};

pub fn how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(input: &mut dyn BufRead, m: usize, n: usize) -> usize {
  let map = CartesianGrid::from(read_input(input));
  let start = map.find_one_coords('S').unwrap();
  let end = map.find_one_coords('E').unwrap();
  let possible_cheats = find_possible_cheats(&map, &start, m);
  let dist = map.find_best_path(&start, &end);

  possible_cheats.iter()
    .flat_map(|(pos, cheats)| cheats.iter()
      .filter(|cheat| *dist.get(&cheat).unwrap() as isize - *dist.get(pos).unwrap() as isize - manhattan_distance(pos, cheat) as isize + 1 >= n as isize)
    )
    .count()
}

fn manhattan_distance(a: &Coords, b: &Coords) -> usize {
  let diff = a - b;
  diff.0.abs() as usize + diff.1.abs() as usize
}

fn find_possible_cheats(map: &CartesianGrid<char>, start: &Coords, distance: usize) -> HashMap<Coords, HashSet<Coords>> {
  let mut possible_cheats: HashMap<Coords, HashSet<Coords>> = HashMap::new();
  let mut visited: HashSet<Coords> = HashSet::new();
  let mut queue: Vec<Coords> = vec![*start];
  
  while let Some(current) = queue.pop() {
    visited.insert(current);
    possible_cheats.insert(current, map.find_possible_cheats(current, distance));

    for neighbor in map.neighbors(&current) {
      if !visited.contains(&neighbor) {
        queue.push(neighbor);
      }
    }
  }
  
  possible_cheats
}

trait RacingMap {
  fn neighbors(&self, coords: &Coords) -> Vec<Coords>;
  fn coords_within_manhattan_distance(&self, coords: &Coords, distance: usize) -> Vec<Coords>;
  fn find_possible_cheats(&self, coords: Coords, distance: usize) -> HashSet<Coords>;
  fn find_best_path(&self, start: &Coords, end: &Coords) -> HashMap<Coords, usize>;
}

#[derive(Eq, PartialEq)]
struct State {
  position: Coords,
  time: usize
}

impl State {
  fn new(position: Coords, time: usize) -> State {
    State {
      position,
      time
    }
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other.time.cmp(&self.time)
  }
}

impl RacingMap for CartesianGrid<char> {
  fn neighbors(&self, coords: &Coords) -> Vec<Coords> {
    self.coords_within_manhattan_distance(coords, 1)
      .iter()
      .filter(|c| *self.get(c) != '#')
      .map(|c| *c)
      .collect_vec()
  }

  fn coords_within_manhattan_distance(&self, coords: &Coords, distance: usize) -> Vec<Coords> {
    (-(distance as isize)..=distance as isize)
      .cartesian_product(-(distance as isize)..=distance as isize)
      .filter(|(dx, dy)| dx.abs() + dy.abs() <= distance as isize)
      .filter(|(dx, dy)| !(*dx == 0 && *dy == 0))
      .map(|(dx, dy)| coords + ICoords(dx, dy))
      .flat_map(|c| c.to_coords())
      .filter(|c| c.in_grid(self))
      .collect()
  }

  fn find_possible_cheats(&self, coords: Coords, distance: usize) -> HashSet<Coords> {
    self.coords_within_manhattan_distance(&coords, distance)
      .iter()
      .filter(|c| *self.get(*c) != '#')
      .map(|c| *c)
      .collect()
  }

  fn find_best_path(&self, start: &Coords, end: &Coords) -> HashMap<Coords, usize> {
    let mut visited: HashSet<Coords> = HashSet::new();
    let mut dist: HashMap<Coords, usize> = HashMap::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State::new(*start, 0));
    dist.insert(*start, 0);
    
    while let Some(s) = queue.pop() {
      if s.position == *end {
        return dist
      }

      visited.insert(s.position);

      for neighbor in self.neighbors(&s.position) {
        let old = dist.get(&neighbor);
        let alt = s.time + 1;

        if !visited.contains(&neighbor) {
          if old.is_none() || alt < *old.unwrap() {
            dist.insert(neighbor, alt);
            queue.push(State::new(neighbor, alt));
          }
        }
      }
    }

    panic!("No path found")
  }
}

#[cfg(test)]
mod tests {
  use crate::{day20::how_many_m_lasting_cheats_would_save_at_least_n_picoseconds, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 2, 64), 1);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 2, 40), 2);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 2, 38), 3);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 2, 36), 4);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 2, 20), 5);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 2, 12), 8);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 2, 10), 10);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 2, 8), 14);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 2, 6), 16);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 2, 4), 30);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 2, 2), 44);
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/my.input"), 2, 100), 1450)
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 76), 3);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 74), 7);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 72), 29);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 70), 41);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 68), 55);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 66), 67);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 64), 86);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 62), 106);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 60), 129);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 58), 154);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 56), 193);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 54), 222);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 52), 253);
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20, 50), 285);
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(how_many_m_lasting_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/my.input"), 20, 100), 1015247)
  }
}