use std::io::BufRead;

use crate::{read_input, Coords, ICoords};

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
    (
      ((robot.position.0 as isize + robot.velocity.0 * period as isize).rem_euclid(self.size.0 as isize)) as usize,
      ((robot.position.1 as isize + robot.velocity.1 * period as isize).rem_euclid(self.size.1 as isize)) as usize
    )
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
      position: (
        captures.name("px").unwrap().as_str().parse::<usize>().unwrap(),
        captures.name("py").unwrap().as_str().parse::<usize>().unwrap()
      ),
      velocity: (
        captures.name("vx").unwrap().as_str().parse::<isize>().unwrap(),
        captures.name("vy").unwrap().as_str().parse::<isize>().unwrap()
      )
    }
  }

  lines.iter().map(|line| parse_robot(line)).collect()
}

#[cfg(test)]
mod tests {
    use crate::{day14::safety_factor, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(safety_factor(&mut read("./src/day14/sample.input"), 100, (11, 7)), 12)
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(safety_factor(&mut read("./src/day14/my.input"), 100, (101, 103)), 223020000)
  }
}