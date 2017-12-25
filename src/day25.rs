use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::Lines;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = indoc!(
      "Begin in state A.
       Perform a diagnostic checksum after 6 steps.

       In state A:
         If the current value is 0:
           - Write the value 1.
           - Move one slot to the right.
           - Continue with state B.
         If the current value is 1:
           - Write the value 0.
           - Move one slot to the left.
           - Continue with state B.

       In state B:
         If the current value is 0:
           - Write the value 1.
           - Move one slot to the left.
           - Continue with state A.
         If the current value is 1:
           - Write the value 1.
           - Move one slot to the right.
           - Continue with state A."
    );
    assert_eq!(3, run1(input));
  }

  #[ignore]
  #[test]
  fn no2_sample_test() {
    assert_eq!(
      9,
      run2(indoc!(
        "5 9 2 8
         9 4 7 3
         3 8 6 5"
      ))
    );
  }

  #[ignore]
  #[test]
  fn no1_test() {
    assert_eq!(2832, no1());
  }

  #[ignore]
  #[test]
  fn no2_test() {
    assert_eq!(191, no2());
  }
}

pub fn no1() -> usize {
  let input = include_str!("../inputs/input25").trim_right();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input25").trim_right();
  run2(input)
}

fn run1(input: &str) -> usize {
  Machine::new(input).perform_diagnostic()
}

fn run2(input: &str) -> i32 {
  0
}

#[derive(Debug)]
struct Machine {
  position: i32,
  ones: HashSet<i32>,
  state: char,
  diagnostic: i32,
  instructions: HashMap<(char, bool), (bool, i32, char)>,
}

impl<'t> Machine {
  fn new(blueprint: &str) -> Machine {
    let mut lines = blueprint.lines();

    let state = Machine::capture_next_line(&mut lines, r"^Begin in state (.)\.$").unwrap().chars().next().unwrap();
    let diagnostic = Machine::capture_next_line(&mut lines, r"^Perform a diagnostic checksum after (\d+) steps.$").unwrap().parse().unwrap();

    let mut instructions = HashMap::new();

    // Discard empty line
    lines.next();

    loop {
      if let Some(curr_state) = Machine::capture_next_line(&mut lines, r"^In state (.):$").map(|i| i.chars().next().unwrap()) {
        loop {
          if let Some(curr_value) = Machine::capture_next_line(&mut lines, r"^  If the current value is (\d+):$").map(|i| i == "1") {
            let new_value = Machine::capture_next_line(&mut lines, r"^    - Write the value (\d+)\.$").unwrap() == "1";
            let direction =
              if Machine::capture_next_line(&mut lines, r"^    - Move one slot to the (.+)\.$").unwrap() == "right" {
                1
              } else {
                -1
              };
            let next_state = Machine::capture_next_line(&mut lines, r"^    - Continue with state (.)\.$").unwrap().chars().next().unwrap();

            instructions.insert((curr_state, curr_value), (new_value, direction, next_state));
          } else {
            break;
          }
        }
      } else {
        break;
      }
    }

    Machine { position: 0, ones: HashSet::new(), state, diagnostic, instructions }
  }

  fn capture_next_line(iter: &'t mut Lines, regex_str: &str) -> Option<String> {
    if let Some(line) = iter.next() {
      let re = Regex::new(regex_str).unwrap();
      if let Some(cap) = re.captures(line) {
        Some(String::from(&cap[1]))
      } else {
        None
      }
    } else {
      None
    }
  }

  fn perform_diagnostic(mut self) -> usize {
    for i in 0..self.diagnostic {
      self.next();
    }
    self.ones.len()
  }

  fn next(&mut self) {
    if let Some(&(val, dir, s)) = self.instructions.get(&(self.state, self.ones.contains(&self.position))) {
      if val {
        self.ones.insert(self.position);
      } else {
        self.ones.remove(&self.position);
      }

      self.position += dir;
      self.state = s;
    }
  }
}
