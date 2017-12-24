use std::collections::HashMap;

#[cfg(test)]
mod tests {
  use super::*;

  #[ignore]
  #[test]
  fn no1_sample_test() {
    let input = indoc!(
      "set a 1
       add a 2
       mul a a
       mod a 5
       snd a
       set a 0
       rcv a
       jgz a -1
       set a 1
       jgz a -2"
    );
    assert_eq!(4, run1(input));
  }

  #[ignore]
  #[test]
  fn no2_sample_test() {
    let input = indoc!(
      "snd 1
       snd 2
       snd p
       rcv a
       rcv b
       rcv c
       rcv d"
    );
    assert_eq!(3, run2(input));
  }

  #[ignore]
  #[test]
  fn no1_test() {
    assert_eq!(1187, no1());
  }

  #[ignore]
  #[test]
  fn no2_test() {
    assert_eq!(5969, no2());
  }
}

pub fn no1() -> i64 {
  let input = include_str!("../inputs/input23").trim_right();
  run1(input)
}

pub fn no2() -> i64 {
  let input = include_str!("../inputs/input23").trim_right();
  run2(input)
}

fn run1(input: &str) -> i64 {
  let mut t = Coprocessor::new(input.to_string());
  t.run().unwrap()
}

fn run2(input: &str) -> i64 {
  let mut t = Coprocessor::new(input.to_string());
  t.run().unwrap()
}

struct Coprocessor {
  registers: HashMap<String, i64>,
  instructions: Vec<Vec<String>>,
  counter: i64,
}

impl Coprocessor {
  fn new(instructions: String) -> Coprocessor {
    Coprocessor {
      registers: HashMap::new(),
      instructions: instructions
        .lines()
        .map(|l| l.split(" ").map(|c| c.to_string()).collect())
        .collect(),
      counter: 0,
    }
  }

  fn run(&mut self) -> Option<i64> {
    loop {
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

  fn get_value(&self, argument: String) -> i64 {
    if let Some(val) = argument.parse().ok() {
      val
    } else {
      *self.registers.get(&argument).unwrap()
    }
  }
}
