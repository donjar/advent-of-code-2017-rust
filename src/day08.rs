use regex::Regex;
use std::cmp;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = indoc!("b inc 5 if a > 1
                        a inc 1 if b < 5
                        c dec -10 if a >= 1
                        c inc -20 if c == 10");
    assert_eq!(1, run1(input));
  }

  #[test]
  fn no2_sample_test() {
    let input = indoc!("b inc 5 if a > 1
                        a inc 1 if b < 5
                        c dec -10 if a >= 1
                        c inc -20 if c == 10");
    assert_eq!(10, run2(input));
  }

  #[test]
  fn no1_test() {
    assert_eq!(5849, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(6702, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input8").trim();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input8").trim();
  run2(input)
}

fn run1(program: &str) -> i32 {
  let mut cpu = Cpu::new();

  for ins in program.lines() {
    cpu.run(ins);
  }

  cpu.get_max()
}

fn run2(program: &str) -> i32 {
  let mut cpu = Cpu::new();
  let mut max = 0;

  for ins in program.lines() {
    cpu.run(ins);
    max = cmp::max(max, cpu.get_max());
  }

  max
}

struct Cpu {
  registers: HashMap<String, i32>,
}

impl Cpu {
  fn new() -> Cpu {
    Cpu { registers: HashMap::new() }
  }

  fn run(&mut self, ins: &str) {
    lazy_static! {
      static ref INCREMENT_REGEX: Regex =
        Regex::new(r"^(.*) inc (.*) if (.*)$").unwrap();
      static ref DECREMENT_REGEX: Regex =
        Regex::new(r"^(.*) dec (.*) if (.*)$").unwrap();
    }

    for captures in INCREMENT_REGEX.captures_iter(ins) {
      let register = &captures[1];
      let increment = captures[2].parse::<i32>();
      let condition = &captures[3];

      if let Ok(inc) = increment {
        self.run_raw(register, inc, condition);
      }
    }

    for captures in DECREMENT_REGEX.captures_iter(ins) {
      let register = &captures[1];
      let decrement = captures[2].parse::<i32>();
      let condition = &captures[3];

      if let Ok(dec) = decrement {
        self.run_raw(register, -1 * dec, condition);
      }
    }
  }

  fn run_raw(&mut self, register: &str, increment: i32, condition: &str) {
    if self.parse_condition(condition) {
      let val = *self.registers.entry(register.to_string()).or_insert(0) + increment;
      self.registers.insert(register.to_string(), val);
    }
  }

  fn parse_condition(&mut self, condition: &str) -> bool {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^(.*) (.*) (.*)$").unwrap();
    }

    for captures in RE.captures_iter(condition) {
      let lhs = *self.registers.entry(captures[1].to_string()).or_insert(0);
      let operator = &captures[2];
      let rhs = captures[3].parse::<i32>().unwrap();

      match operator {
        ">=" => return lhs >= rhs,
        ">" => return lhs > rhs,
        "<=" => return lhs <= rhs,
        "<" => return lhs < rhs,
        "==" => return lhs == rhs,
        "!=" => return lhs != rhs,
        _ => assert!(false),
      }
    }
    assert!(false);
    false
  }

  fn get_max(&self) -> i32 {
    *self.registers.values().max().unwrap()
  }
}
