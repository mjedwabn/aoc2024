use itertools::Itertools;
use std::{collections::HashMap, io::BufRead};

use crate::read_input;

type Rule = (u32, u32);
type Update = Vec<u32>;

pub fn sum_middle_page_numbers_of_correct_updates(input: &mut dyn BufRead) -> u32 {
  let (rules, updates) = parse_input(read_input(input));
  updates
    .iter()
    .filter(|u| is_correctly_ordered(u, &rules))
    .map(|u| get_middle_page(u))
    .sum()
}

pub fn sum_middle_page_numbers_of_corrected_updates(input: &mut dyn BufRead) -> u32 {
  let (rules, updates) = parse_input(read_input(input));
  updates
    .iter()
    .filter(|u| !is_correctly_ordered(u, &rules))
    .map(|u| correct_the_update(u, &rules))
    .map(|u| *get_middle_page(&u))
    .sum()
}

fn parse_input(lines: Vec<String>) -> (Vec<Rule>, Vec<Update>) {
  fn parse_rule(line: &String) -> Rule {
    line
      .split_once('|')
      .map(|x| (x.0.parse::<u32>().unwrap(), x.1.parse::<u32>().unwrap()))
      .unwrap()
  }

  fn parse_update(line: &String) -> Vec<u32> {
    line
      .split(',')
      .map(|page| page.parse::<u32>().unwrap())
      .collect()
  }

  let sections = lines
    .split(|line| line == "")
    .map(|ll| ll.iter().map(String::from).collect())
    .collect::<Vec<Vec<String>>>();
  let rules = sections
    .first()
    .unwrap()
    .iter()
    .map(|line| parse_rule(line))
    .collect::<Vec<Rule>>();
  let updates = sections
    .last()
    .unwrap()
    .iter()
    .map(|line| parse_update(line))
    .collect::<Vec<Update>>();

  (rules, updates)
}

fn is_correctly_ordered(update: &Update, rules: &Vec<Rule>) -> bool {
  update
    .iter()
    .enumerate()
    .all(|page| is_correctly_placed(page.0, update, rules))
}

fn is_correctly_placed(page_number: usize, update: &Update, rules: &Vec<Rule>) -> bool {
  (page_number + 1..update.len()).all(|after| {
    page_follows_rules(
      update.get(page_number).unwrap(),
      update.get(after).unwrap(),
      rules,
    )
  })
}

fn page_follows_rules(page: &u32, against: &u32, rules: &Vec<Rule>) -> bool {
  rules
    .iter()
    .filter(|r| rule_applies(page, against, r))
    .all(|r| page_follows_rule(page, against, r))
}

fn rule_applies(page: &u32, against: &u32, rule: &Rule) -> bool {
  (*page == rule.0 || *page == rule.1) && (*against == rule.0 || *against == rule.1)
}

fn page_follows_rule(page: &u32, against: &u32, rule: &Rule) -> bool {
  *page == rule.0 && *against == rule.1
}

fn get_middle_page(update: &Update) -> &u32 {
  update.get(update.len() / 2).unwrap()
}

fn correct_the_update(update: &Update, rules: &Vec<Rule>) -> Update {
  let graph = make_rules_graph(rules);
  let mut corrected = update.clone();

  for i in (0..corrected.len()).rev() {
    let to_check = corrected.iter().take(i + 1).collect::<Vec<&u32>>();
    let (pos, _) = to_check
      .iter()
      .find_position(|v| has_no_deps(v, &values_except(v, &to_check), &graph))
      .unwrap();
    corrected.swap(i, pos);
  }

  corrected
}

fn values_except<'a>(e: &u32, values: &'a Vec<&u32>) -> Vec<&'a u32> {
  values
    .iter()
    .filter(|&&v| v != e)
    .map(|x| *x)
    .collect::<Vec<&u32>>()
}

fn has_no_deps(v: &u32, on: &Vec<&u32>, graph: &HashMap<u32, Vec<u32>>) -> bool {
  graph
    .get(v)
    .map(|successors| on.iter().all(|o| !successors.contains(o)))
    .unwrap_or(true)
}

fn make_rules_graph(rules: &Vec<Rule>) -> HashMap<u32, Vec<u32>> {
  rules
    .iter()
    .into_group_map_by(|&r| r.0)
    .into_iter()
    .map(|(pred, rs)| (pred, rs.iter().map(|r| r.1).collect()))
    .collect()
}

#[cfg(test)]
mod tests {
  use std::{fs::File, io::BufReader};

  use crate::day05::{
    sum_middle_page_numbers_of_correct_updates, sum_middle_page_numbers_of_corrected_updates,
  };

  #[test]
  fn sample_part1_input() {
    let mut f = BufReader::new(File::open("./src/day05/sample.input").unwrap());
    assert_eq!(sum_middle_page_numbers_of_correct_updates(&mut f), 143)
  }

  #[test]
  fn my_part1_input() {
    let mut f = BufReader::new(File::open("./src/day05/my.input").unwrap());
    assert_eq!(sum_middle_page_numbers_of_correct_updates(&mut f), 6034)
  }

  #[test]
  fn sample_part2_input() {
    let mut f = BufReader::new(File::open("./src/day05/sample.input").unwrap());
    assert_eq!(sum_middle_page_numbers_of_corrected_updates(&mut f), 123)
  }

  #[test]
  fn my_part2_input() {
    let mut f = BufReader::new(File::open("./src/day05/my.input").unwrap());
    assert_eq!(sum_middle_page_numbers_of_corrected_updates(&mut f), 6305)
  }
}
