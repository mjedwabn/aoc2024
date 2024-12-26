use std::{collections::{BinaryHeap, HashMap, HashSet}, io::BufRead};

use itertools::Itertools;

use crate::{read_input, CartesianGrid, Coords, GridCoords, ICoords};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct State {
  position: Coords,
  direction: ICoords,
  cost: u32
}

impl State {
  fn new(position: Coords, direction: ICoords, cost: u32) -> State {
    State {
      position,
      direction,
      cost
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
    other.cost.cmp(&self.cost)
  }
}

pub fn lowest_score_path(input: &mut dyn BufRead) -> u32 {
  visit_maze(CartesianGrid::from(read_input(input))).3
}

pub fn best_path_fields(input: &mut dyn BufRead) -> usize {
  let (end, dist, predecessors, min) = visit_maze(CartesianGrid::from(read_input(input)));
  let ends: Vec<(Coords, ICoords)> = (*dist.iter()
    .filter(|((pos, _), _)| *pos == end)
    .filter(|(_, cost)| **cost == min)
    .map(|(c, _)| *c)
    .collect_vec()).to_vec();
  
  let mut visited: HashSet<Coords> = HashSet::new();
  
  for e in ends {
    let mut queue: Vec<(Coords, ICoords)> = Vec::new();
    queue.push(e);

    while let Some(node) = queue.pop() {
      visited.insert(node.0);

      if let Some(preds) = predecessors.get(&node) {
        for p in preds.clone() {
          queue.push(p);
        }
      }
    }
  }

  visited.len()
}

fn visit_maze(maze: CartesianGrid<char>) -> (Coords, HashMap<(Coords, ICoords), u32>, HashMap<(Coords, ICoords), HashSet<(Coords, ICoords)>>, u32) {
  let start = maze.find_one_coords('S').unwrap();
  let end = maze.find_one_coords('E').unwrap();

  let mut dist: HashMap<(Coords, ICoords), u32> = HashMap::new();
  let mut prev: HashMap<(Coords, ICoords), HashSet<(Coords, ICoords)>> = HashMap::new();
  let mut queue: BinaryHeap<State> = BinaryHeap::new();
  let mut visited: HashSet<(Coords, ICoords)> = HashSet::new();
  
  queue.push(State::new(start, ICoords(1, 0), 0));

  while let Some(curr) = queue.pop() {
    visited.insert((curr.position, curr.direction));

    for (next, next_dir) in maze.neighbors(&curr.position).iter()
      .map(|n| (*n, maze.dir_after(&curr.position, n)))
      .filter(|n| !visited.contains(n)) 
    {
      let alt = curr.cost + maze.cost(&curr.position, &curr.direction, &next);
      let old = dist.insert((next, next_dir), alt);

      if old.is_none() || alt < old.unwrap() {
        prev.insert((next, next_dir), HashSet::new());
      }

      prev.entry((next, next_dir))
        .and_modify(|s| { s.insert((curr.position, curr.direction)); })
        .or_insert(HashSet::new());
      dist.insert((next, next_dir), alt);
      queue.push(State::new(next, maze.dir_after(&curr.position, &next), alt));
    }
  }

  let min = *dist.iter()
    .filter(|((pos, _), _)| *pos == end)
    .map(|(_, cost)| cost)
    .min()
    .unwrap();

  (end, dist, prev, min)
}

impl CartesianGrid<char> {
  fn neighbors(&self, coords: &Coords, ) -> Vec<Coords> {
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

  fn cost(&self, from: &Coords, direction: &ICoords, to: &Coords) -> u32 {
    if self.dir_after(from, to) == *direction {
      1
    }
    else {
      1001
    }
  }

  fn dir_after(&self, from: &Coords, to: &Coords) -> ICoords {
    to - from
  }
}

#[cfg(test)]
mod tests {
    use crate::{day16::{lowest_score_path, best_path_fields}, read};

  #[test]
  fn first_sample_part1_input() {
    assert_eq!(lowest_score_path(&mut read("./src/day16/first.sample.input")), 7036)
  }

  #[test]
  fn second_sample_part1_input() {
    assert_eq!(lowest_score_path(&mut read("./src/day16/second.sample.input")), 11048)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(lowest_score_path(&mut read("./src/day16/my.input")), 143564)
  }

  #[test]
  fn first_sample_part2_input() {
    assert_eq!(best_path_fields(&mut read("./src/day16/first.sample.input")), 45)
  }

  #[test]
  fn second_sample_part2_input() {
    assert_eq!(best_path_fields(&mut read("./src/day16/second.sample.input")), 64)
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(best_path_fields(&mut read("./src/day16/my.input")), 593)
  }
}