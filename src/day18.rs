use std::collections::HashMap;
use std::collections::VecDeque;

#[cfg(test)]
mod tests {
  use super::*;

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

  #[test]
  fn no1_test() {
    assert_eq!(1187, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(5969, no2());
  }
}

pub fn no1() -> i64 {
  let input = include_str!("../inputs/input18").trim_right();
  run1(input)
}

pub fn no2() -> i64 {
  let input = include_str!("../inputs/input18").trim_right();
  run2(input)
}

fn run1(input: &str) -> i64 {
  let mut t = SoundTablet::new(input.to_string());
  t.run().unwrap()
}

fn run2(input: &str) -> i64 {
  let mut t = MqTablet::new(input.to_string());
  t.run()
}

struct SoundTablet {
  registers: HashMap<String, i64>,
  instructions: Vec<Vec<String>>,
  counter: i64,
  last_sound: Option<i64>,
}

impl SoundTablet {
  fn new(instructions: String) -> SoundTablet {
    SoundTablet {
      registers: HashMap::new(),
      instructions: instructions
        .lines()
        .map(|l| l.split(" ").map(|c| c.to_string()).collect())
        .collect(),
      counter: 0,
      last_sound: None,
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
        "add" => {
          let second = self.get_value(current[2].clone());
          let current = *self.registers.get(&first).unwrap_or(&0);
          self.registers.insert(first, current + second);
        }
        "mul" => {
          let second = self.get_value(current[2].clone());
          let current = *self.registers.get(&first).unwrap_or(&0);
          self.registers.insert(first, current * second);
        }
        "mod" => {
          let second = self.get_value(current[2].clone());
          let current = *self.registers.get(&first).unwrap_or(&0);
          self.registers.insert(first, current % second);
        }
        "snd" => {
          self.last_sound = Some(self.get_value(first));
        }
        "rcv" => {
          if self.get_value(first) != 0 {
            return self.last_sound;
          }
        }
        "jgz" => {
          if self.get_value(first) > 0 {
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

struct MqTablet {
  registers0: HashMap<String, i64>,
  registers1: HashMap<String, i64>,
  queue0: VecDeque<i64>,
  queue1: VecDeque<i64>,
  instructions: Vec<Vec<String>>,
  counter0: i64,
  counter1: i64,
  turn: i64,
  block: i64,
}

impl MqTablet {
  fn new(instructions: String) -> MqTablet {
    let mut r0 = HashMap::new();
    r0.insert(String::from("p"), 0);
    let mut r1 = HashMap::new();
    r1.insert(String::from("p"), 1);

    MqTablet {
      registers0: r0,
      registers1: r1,
      queue0: VecDeque::new(),
      queue1: VecDeque::new(),
      instructions: instructions
        .lines()
        .map(|l| l.split(" ").map(|c| c.to_string()).collect())
        .collect(),
      counter0: 0,
      counter1: 0,
      turn: 0,
      block: 0,
    }
  }

  fn run(&mut self) -> i64 {
    let mut p1_count = 0;

    loop {
      let counter = self.current_counter().clone() as usize;
      let current = &self.instructions[counter].clone();
      let first = current[1].clone();

      match current[0].as_ref() {
        "set" => {
          let second = self.get_value(current[2].clone());
          self.current_registers().insert(first, second);
        }
        "add" => {
          let second = self.get_value(current[2].clone());
          let current = *self.current_registers().get(&first).unwrap_or(&0);
          self.current_registers().insert(first, current + second);
        }
        "mul" => {
          let second = self.get_value(current[2].clone());
          let current = *self.current_registers().get(&first).unwrap_or(&0);
          self.current_registers().insert(first, current * second);
        }
        "mod" => {
          let second = self.get_value(current[2].clone());
          let current = *self.current_registers().get(&first).unwrap_or(&0);
          self.current_registers().insert(first, current % second);
        }
        "snd" => {
          let val = self.get_value(first);
          self.other_queue().push_back(val);
          if self.turn == 1 {
            p1_count += 1;
          }
        }
        "rcv" => {
          if self.rcv(first.clone()).is_err() {
            if self.block > 0 {
              return p1_count;
            } else {
              self.switch_program();
              *self.current_counter() -= 1; // negate counter addition
              self.block = 2; // 2 blocks and you're out
            }
          }
        }
        "jgz" => {
          if self.get_value(first) > 0 {
            let second = self.get_value(current[2].clone());
            *self.current_counter() += second;
            *self.current_counter() -= 1; // negate counter addition
          }
        }
        _ => panic!(format!("Instruction {:?} not recognized", current)),
      }

      *self.current_counter() += 1;
      if self.block > 0 {
        self.block -= 1;
      }
    }
  }

  fn get_value(&mut self, argument: String) -> i64 {
    if let Some(val) = argument.parse().ok() {
      val
    } else {
      *self.current_registers().get(&argument).unwrap()
    }
  }

  fn current_registers(&mut self) -> &mut HashMap<String, i64> {
    match self.turn {
      0 => &mut self.registers0,
      1 => &mut self.registers1,
      _ => panic!("Invalid turn"),
    }
  }

  fn current_counter(&mut self) -> &mut i64 {
    match self.turn {
      0 => &mut self.counter0,
      1 => &mut self.counter1,
      _ => panic!("Invalid turn"),
    }
  }

  fn current_queue(&mut self) -> &mut VecDeque<i64> {
    match self.turn {
      0 => &mut self.queue0,
      1 => &mut self.queue1,
      _ => panic!("Invalid turn"),
    }
  }

  fn other_queue(&mut self) -> &mut VecDeque<i64> {
    match self.turn {
      0 => &mut self.queue1,
      1 => &mut self.queue0,
      _ => panic!("Invalid turn"),
    }
  }

  fn switch_program(&mut self) {
    match self.turn {
      0 => self.turn = 1,
      1 => self.turn = 0,
      _ => panic!("Invalid turn"),
    }
  }

  fn rcv(&mut self, register: String) -> Result<(), ()> {
    let msg = self.current_queue().pop_front();

    if let Some(m) = msg {
      self.current_registers().insert(register, m);
      Ok(())
    } else {
      Err(())
    }
  }
}
