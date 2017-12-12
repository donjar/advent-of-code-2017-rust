use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::Sub;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = indoc!(
      "pbga (66)
       xhth (57)
       ebii (61)
       havc (66)
       ktlj (57)
       fwft (72) -> ktlj, cntj, xhth
       qoyq (66)
       padx (45) -> pbga, havc, qoyq
       tknk (41) -> ugml, padx, fwft
       jptl (61)
       ugml (68) -> gyxo, ebii, jptl
       gyxo (61)
       cntj (57)"
    );
    assert_eq!("tknk", run1(input));
  }

  #[test]
  fn no2_sample_test() {
    let input = indoc!(
      "pbga (66)
       xhth (57)
       ebii (61)
       havc (66)
       ktlj (57)
       fwft (72) -> ktlj, cntj, xhth
       qoyq (66)
       padx (45) -> pbga, havc, qoyq
       tknk (41) -> ugml, padx, fwft
       jptl (61)
       ugml (68) -> gyxo, ebii, jptl
       gyxo (61)
       cntj (57)"
    );
    assert_eq!(8, run2(input));
  }

  #[test]
  fn no1_test() {
    assert_eq!("hlqnsbe", no1());
  }

  #[ignore]
  #[test]
  fn no2_test() {
    assert_eq!(191, no2());
  }
}

pub fn no1() -> String {
  let input = include_str!("../inputs/input07").trim();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input07").trim();
  0
}

fn run1(list: &str) -> String {
  let re = Regex::new(r"^(.*) \(.*\)( -> )?(.*)$").unwrap();

  let mut all_programs = HashSet::new();
  let mut top_programs = HashSet::new();

  for line in list.lines() {
    for cap in re.captures_iter(line) {
      all_programs.insert(cap[1].to_string());

      for prog in cap[3].split(", ") {
        top_programs.insert(prog.to_string());
      }
    }
  }

  all_programs.sub(&top_programs).into_iter().nth(0).unwrap()
}

fn run2(list: &str) -> i32 {
  0
}

#[derive(PartialEq, Eq, Hash)]
struct Program<'a> {
  name: String,
  children: Vec<&'a Program<'a>>,
  weight: i32,
  sum: i32,
}

struct ProgramSystem<'a> {
  programs: HashMap<String, Program<'a>>,
}

impl<'a> Program<'a> {
  fn empty_program(name: String) -> Program<'a> {
    Program {
      name,
      children: Vec::new(),
      weight: 0,
      sum: 0,
    }
  }
}

impl<'a> ProgramSystem<'a> {
  fn get_roots(&self) -> HashSet<&Program> {
    let mut all_programs = HashSet::new();
    let mut childrens = HashSet::new();

    for p in self.programs.values() {
      all_programs.insert(p);
      for c in p.children.iter() {
        childrens.insert(c);
      }
    }

    all_programs.sub(childrens)
  }

  fn insert_program(&mut self, code: &str) {
    let code_regex = Regex::new(r"^(.*) \((.*)\)( -> )?(.*)$").unwrap();

    for cap in code_regex.captures_iter(code) {
      let name = cap[1].to_string();
      let weight = cap[2].parse().unwrap();
      let children = cap[4].split(", ");

      let program = self.register_program(name, weight);

      for child in children {
        let child_string = child.to_string();

        let empty_child_program = Program::empty_program(child_string.clone());
        let child_program = self.programs.entry(child_string).or_insert(empty_child_program);
        program.children.push(child_program);
      }
    }
  }

  fn register_program(&mut self, name: String, weight: i32) -> &mut Program {
    let empty_program = Program::empty_program(name.clone());
    let program = self.programs.entry(name).or_insert(empty_program);
    program.weight = weight;
    program
  }
}
