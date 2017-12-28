use std::collections::HashMap;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_test() {
    assert_eq!(9409, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(913, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input23").trim_right();
  run1(input)
}

pub fn no2() -> i32 {
  run2()
}

fn run1(input: &str) -> i32 {
  let mut t = Coprocessor::new(input.to_string(), 0);
  t.run()
}

fn run2() -> i32 {
  let inc = 17;
  let total = 1001;

  let b = 99 * 100 + 100000;
  let c = b + inc * total;
  let m = b % inc;
  let mut ps = 0;

  for p in primes(c) {
    if p >= b && p % inc == m {
      ps += 1;
    }
  }

  total - ps
}

fn primes(below: i32) -> Vec<i32> {
  let mut res = Vec::new();

  for i in 2..below {
    let mut is_prime = true;

    for p in res.iter() {
      if i % p == 0 {
        is_prime = false;
        break;
      }
    }

    if is_prime {
      res.push(i);
    }
  }

  res
}

struct Coprocessor {
  registers: HashMap<String, i32>,
  instructions: Vec<Vec<String>>,
  counter: i32,
}

impl Coprocessor {
  fn new(instructions: String, a_value: i32) -> Coprocessor {
    let mut registers = HashMap::new();
    registers.insert(String::from("a"), a_value);
    for &r in ["b", "c", "d", "e", "f", "g", "h"].iter() {
      registers.insert(String::from(r), 0);
    }

    Coprocessor {
      registers,
      instructions: instructions
        .lines()
        .map(|l| l.split(" ").map(String::from).collect())
        .collect(),
      counter: 0,
    }
  }

  fn run(&mut self) -> i32 {
    let mut mul_count = 0;
    loop {
      if self.counter < 0 || self.counter as usize >= self.instructions.len() {
        return mul_count;
      }

      let current = &self.instructions[self.counter as usize];
      let first = current[1].clone();

      match current[0].as_ref() {
        "set" => {
          let second = self.get_value(current[2].clone());
          self.registers.insert(first, second);
        }
        "sub" => {
          let second = self.get_value(current[2].clone());
          let current = *self.registers.get(&first).unwrap_or(&0);
          self.registers.insert(first, current - second);
        }
        "mul" => {
          let second = self.get_value(current[2].clone());
          let current = *self.registers.get(&first).unwrap_or(&0);
          self.registers.insert(first, current * second);
          mul_count += 1;
        }
        "jnz" => {
          if self.get_value(first) != 0 {
            let second = self.get_value(current[2].clone());
            self.counter += second;
            self.counter -= 1; // negate counter addition
          }
        }
        _ => panic!(format!("Instruction {:?} not recognized", current)),
      }

      self.counter += 1;
    }
  }

  fn get_value(&self, argument: String) -> i32 {
    if let Some(val) = argument.parse().ok() {
      val
    } else {
      *self.registers.get(&argument).unwrap()
    }
  }
}
