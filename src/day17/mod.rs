use std::io::BufRead;

use itertools::Itertools;

use crate::read_input;

type Num = u64;

pub fn final_output(input: &mut dyn BufRead) -> String {
  let (a, b, c, program) = parse_input(read_input(input));

  let mut computer = Computer::new(a, b, c, program.clone());
  computer.run();

  computer.output()
}

pub fn reproduce_itself(input: &mut dyn BufRead) -> Num {
  let (a, b, c, program) = parse_input(read_input(input));

  let mut computer = Computer::new(a, b, c, program);

  (0..).find(|&a| {
    computer.reset(a, b, c);
    computer.run();
    computer.has_reproduced_program()
  }).expect("Solution not found!")
}

//bst 2, 4   -> b = a & 0b111 = xyz
//bxl 1, 5   -> b = b xor 5 = b xor 0b101
//cdv 7, 5   -> c = a >> b
//bxl 1, 6   -> b = b xor 6 = b xor 0b110
//adv 0, 3   -> a = a >> 3
//bxc 4, 6   -> b = b xor c
//out 5, 5   -> out b & 0b111
//jnz 3, 0   -> if a != 0 goto 0

pub fn reproduce_itself_v2(input: &mut dyn BufRead) -> Num {
  let (_, b, c, program) = parse_input(read_input(input));
  
  (0..program.len()).rev().fold(vec![0], |bases, i| {
    bases.into_iter().flat_map(|base| {
      let program = &program;
      (0..=7).filter_map(move |j| {
        let a = (base << 3) + j as Num;
        if subprogram_was_reproduced(a, b, c, &program, i) {
          Some(a)
        } else {
          None
        }
      })
    }).collect()
  }).into_iter().min().expect("Unable to reproduce program")
}

fn subprogram_was_reproduced(a: u64, b: u64, c: u64, program: &Vec<u64>, i: usize) -> bool {
  let mut computer = Computer::new(a, b, c, program.clone());
  computer.run();
  computer.output == program[i..]
}

fn parse_input(lines: Vec<String>) -> (Num, Num, Num, Vec<Num>) {
  let a = lines[0].split_once(':').unwrap().1.trim().parse::<Num>().unwrap();
  let b = lines[1].split_once(':').unwrap().1.trim().parse::<Num>().unwrap();
  let c = lines[2].split_once(':').unwrap().1.trim().parse::<Num>().unwrap();
  let program = lines[4].split_once(':').unwrap().1.trim().split(',').flat_map(|n| n.parse::<Num>()).collect_vec();

  (a, b, c, program)
}

pub struct Computer {
  a: Num,
  b: Num,
  c: Num,
  program: Vec<Num>,
  eip: usize,
  pub output: Vec<Num>
}

impl Computer {
  pub fn new(a: Num, b: Num, c: Num, program: Vec<Num>) -> Computer {
    Computer {
      a,
      b,
      c,
      program,
      eip: 0,
      output: Vec::new()
    }
  }

  pub fn run(&mut self) {
    while !self.is_halt() {
      let opcode = self.program[self.eip];
      let operand = self.program[self.eip + 1];
      self.eip += 2;

      match opcode {
          0 => self.a >>= self.combo(operand),
          1 => self.b ^= self.literal(operand),
          2 => self.b = self.combo(operand) & 0b111,
          3 => if self.a != 0 { self.eip = self.literal(operand) as usize },
          4 => self.b ^= self.c,
          5 => self.output.push(self.combo(operand) & 0b111),
          6 => self.b = self.a >> self.combo(operand),
          7 => self.c = self.a >> self.combo(operand),
          _ => panic!("Unsupported opcode {}", opcode)
      }
    }
  }

  fn combo(&self, operand: Num) -> Num {
    match operand {
      0..=3 => operand as Num,
      4 => self.a,
      5 => self.b,
      6 => self.c,
      _ => panic!("Reserved operand"),
    }
  }

  fn literal(&self, operand: Num) -> Num {
    operand as Num
  }
  
  fn is_halt(&self) -> bool {
    self.eip >= self.program.len() - 1
  }

  fn has_reproduced_program(&self) -> bool {
    self.output == self.program
  }

  fn output(&self) -> String {
    self.output.clone().into_iter().join(",")
  }

  pub fn reset(&mut self, a: Num, b: Num, c: Num) {
    self.a = a;
    self.b = b;
    self.c = c;
    self.eip = 0;
    self.output.clear();
  }
}

#[cfg(test)]
mod tests {
  use crate::{day17::{final_output, reproduce_itself, reproduce_itself_v2}, read};
  use super::Computer;

  #[test]
  fn sample_part1_input() {
    assert_eq!(final_output(&mut read("./src/day17/sample.input")), "4,6,3,5,6,3,5,2,1,0")
  }

  #[test]
  fn unit1() {
    let mut c = Computer::new(0, 0, 9, vec![2, 6]);
    c.run();
    assert_eq!(c.b, 1);
  }

  #[test]
  fn unit2() {
    let mut c = Computer::new(10, 0, 0, vec![5,0,5,1,5,4]);
    c.run();
    assert_eq!(c.output(), "0,1,2");
  }

  #[test]
  fn unit3() {
    let mut c = Computer::new(2024, 0, 0, vec![0,1,5,4,3,0]);
    c.run();
    assert_eq!(c.output(), "4,2,5,6,7,7,7,7,3,1,0");
    assert_eq!(c.a, 0);
  }

  #[test]
  fn unit4() {
    let mut c = Computer::new(0, 29, 0, vec![1,7]);
    c.run();
    assert_eq!(c.b, 26);
  }

  #[test]
  fn unit5() {
    let mut c = Computer::new(0, 2024, 43690, vec![4,0]);
    c.run();
    assert_eq!(c.b, 44354);
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(final_output(&mut read("./src/day17/my.input")), "3,6,3,7,0,7,0,3,0")
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(reproduce_itself(&mut read("./src/day17/sample.part2.input")), 117440)
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(reproduce_itself_v2(&mut read("./src/day17/my.input")), 136904920099226)
  }
}