use std::io::BufRead;

use itertools::Itertools;

use crate::{read_input, CartesianGrid, Coords, ICoords};

pub fn safety_factor(input: &mut dyn BufRead, period: usize, size: (usize, usize)) -> usize {
  let robots: Vec<Robot> = parse_robots(read_input(input));
  let simulator = Simulator {
    robots,
    size
  };

  let positions = simulator.predict_positions_after(period);
  let quadrants = count_robots_in_quadrants(positions, size);

  quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

pub fn find_easter_egg(input: &mut dyn BufRead, period: usize, size: (usize, usize)) -> usize {
  let robots: Vec<Robot> = parse_robots(read_input(input));
  let simulator = Simulator {
    robots,
    size
  };

  for t in 0..period {
    let positions = simulator.predict_positions_after(t);
    let mut grid = CartesianGrid::empty(size);


    for (p, n) in positions
      .iter()
      .into_group_map_by(|p| *p)
      .into_iter()
      .map(|x| (x.0, x.1.len())) 
    {
      grid.set(p, char::from_digit(n as u32, 10).unwrap());
    }

    println!("t = {} ========================================================================= ", t);
    grid.print_positions();
  }
  
  0
}

impl CartesianGrid<char> {
  pub fn empty(size: (usize, usize)) -> Self {
    CartesianGrid { grid: (0..size.1).map(|_| vec![' '; size.0]).collect() }
  }

  fn print_positions(&self) {
    for level in self.grid.iter() {
      for c in level {
        print!("{}", c);
      }
      println!();
    }
  }
}

fn count_robots_in_quadrants(positions: Vec<Coords>, size: (usize, usize)) -> (usize, usize, usize, usize) {
  (
    positions.iter().filter(|p| p.0 < size.0 / 2 && p.1 < size.1 / 2).count(), 
    positions.iter().filter(|p| p.0 > size.0 / 2 && p.1 < size.1 / 2).count(), 
    positions.iter().filter(|p| p.0 < size.0 / 2 && p.1 > size.1 / 2).count(), 
    positions.iter().filter(|p| p.0 > size.0 / 2 && p.1 > size.1 / 2).count(), 
  )
}

struct Simulator {
  robots: Vec<Robot>,
  size: (usize, usize)
}

impl Simulator {
  fn predict_positions_after(&self, period: usize) -> Vec<Coords> {
    self.robots.iter().map(|r| self.predict_position_after(r, period)).collect()
  }

  fn predict_position_after(&self, robot: &Robot, period: usize) -> Coords {
    (robot.position + robot.velocity * period).rem_euclid(self.size.0, self.size.1)
  }
}

struct Robot {
  position: Coords,
  velocity: ICoords
}

fn parse_robots(lines: Vec<String>) -> Vec<Robot> {
  fn parse_robot(line: &String) -> Robot {
    let re = regex::Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();

    let captures = re.captures(line).unwrap();

    Robot {
      position: Coords::new(
        captures.name("px").unwrap().as_str().parse::<usize>().unwrap(),
        captures.name("py").unwrap().as_str().parse::<usize>().unwrap()
      ),
      velocity: ICoords::new(
        captures.name("vx").unwrap().as_str().parse::<isize>().unwrap(),
        captures.name("vy").unwrap().as_str().parse::<isize>().unwrap()
      )
    }
  }

  lines.iter().map(|line| parse_robot(line)).collect()
}

#[cfg(test)]
mod tests {
    use crate::{day14::{find_easter_egg, safety_factor}, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(safety_factor(&mut read("./src/day14/sample.input"), 100, (11, 7)), 12)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(safety_factor(&mut read("./src/day14/my.input"), 100, (101, 103)), 223020000)
  }

  #[test]
  fn my_part2_input() {
    find_easter_egg(&mut read("./src/day14/my.input"), 7500, (101, 103));
    // 7338
  }
}