use std::{collections::{BinaryHeap, HashMap, HashSet}, io::BufRead};

use itertools::Itertools;

use crate::{read_input, CartesianGrid, Coords, GridCoords, ICoords};

pub fn how_many_cheats_would_save_at_least_n_picoseconds(input: &mut dyn BufRead, n: usize) -> usize {
  let map = CartesianGrid::from(read_input(input));
  let start = map.find_one_coords('S').unwrap();
  let end = map.find_one_coords('E').unwrap();
  let possible_cheats = find_possible_cheats(&map, &start);
  let dist = map.find_best_path(&start, &end);

  possible_cheats.iter()
    .flat_map(|(pos, cheats)| cheats.iter()
      .filter(|cheat| *dist.get(&cheat.1).unwrap() as isize - *dist.get(pos).unwrap() as isize - 1 >= n as isize))
    .count()
}

fn find_possible_cheats(map: &CartesianGrid<char>, start: &Coords) -> HashMap<Coords, HashSet<(Coords, Coords)>> {
  let mut possible_cheats: HashMap<Coords, HashSet<(Coords, Coords)>> = HashMap::new();
  let mut visited: HashSet<Coords> = HashSet::new();
  let mut queue: Vec<Coords> = vec![*start];
  
  while let Some(current) = queue.pop() {
    visited.insert(current);
    possible_cheats.insert(current, map.find_possible_cheats(current));

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
  fn find_possible_cheats(&self, coords: Coords) -> HashSet<(Coords, Coords)>;
  fn cheat(&self, coords: &Coords, direction: &ICoords) -> Option<(Coords, Coords)>;
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
    vec![
      coords.sub_y(1),
      coords.add_x(1),
      coords.add_y(1),
      coords.sub_x(1)
    ]
    .iter()
    .filter(|c| c.in_grid(self))
    .flat_map(|c| c.to_coords())
    .filter(|c| *self.get(c) != '#')
    .collect_vec()
  }

  fn find_possible_cheats(&self, coords: Coords) -> HashSet<(Coords, Coords)> {
    vec![
      ICoords(0, -1),
      ICoords(1, 0),
      ICoords(0, 1),
      ICoords(-1, 0)
    ]
    .iter()
    .flat_map(|d| self.cheat(&coords, d))
    .collect()
  }

  fn cheat(&self, coords: &Coords, direction: &ICoords) -> Option<(Coords, Coords)> {
    let c1 = coords + direction;
    let c2 = coords + direction * 2;
    
    if self.geti(&c1).is_some_and(|c| *c == '#') && self.geti(&c2).is_some_and(|c| *c != '#') {
      Some((c1.to_coords().unwrap(), c2.to_coords().unwrap()))
    }
    else {
      None
    }
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
  use crate::{day20::how_many_cheats_would_save_at_least_n_picoseconds, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(how_many_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 64), 1);
    assert_eq!(how_many_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 40), 2);
    assert_eq!(how_many_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 38), 3);
    assert_eq!(how_many_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 36), 4);
    assert_eq!(how_many_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 20), 5);
    assert_eq!(how_many_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 12), 8);
    assert_eq!(how_many_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 10), 10);
    assert_eq!(how_many_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 8), 14);
    assert_eq!(how_many_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 6), 16);
    assert_eq!(how_many_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 4), 30);
    assert_eq!(how_many_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/sample.input"), 2), 44);
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(how_many_cheats_would_save_at_least_n_picoseconds(&mut read("./src/day20/my.input"), 100), 1450)
  }
}