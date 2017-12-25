use helper::{Coordinate, Direction};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = indoc!(
      "..#
       #..
       ..."
    );
    assert_eq!(5, run1(input, 7));
    assert_eq!(41, run1(input, 70));
    assert_eq!(5587, run1(input, 10000));
  }

  #[ignore]
  #[test]
  fn no2_sample_test() {
    let input = indoc!(
      "..#
       #..
       ..."
    );
    assert_eq!(26, run2(input, 100));
    assert_eq!(2511944, run2(input, 10000000));
  }

  #[test]
  fn no1_test() {
    assert_eq!(5460, no1());
  }

  #[ignore]
  #[test]
  fn no2_test() {
    assert_eq!(2511702, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input22").trim_right();
  run1(input, 10000)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input22").trim_right();
  run2(input, 10000000)
}

fn run1(map: &str, iterations: i32) -> i32 {
  let mut i = Infection::new(map);
  for _ in 0..iterations {
    i.next();
  }
  i.count_infected
}

fn run2(map: &str, iterations: i32) -> i32 {
  let mut i = Infection::new(map);
  for _ in 0..iterations {
    i.strong_next();
  }
  i.count_infected
}

enum InfectionStatus {
  Weakened,
  Infected,
  Flagged,
}

struct Infection {
  status: HashMap<Coordinate<i32>, InfectionStatus>,
  position: Coordinate<i32>,
  direction: Direction,
  count_infected: i32,
}

impl Infection {
  fn new(map: &str) -> Infection {
    let matrix: Vec<Vec<char>> =
      map.lines().map(|l| l.chars().collect()).collect();
    let height = matrix.len();
    let width = matrix[0].len();

    let mut status = HashMap::new();
    for i in 0..height {
      for j in 0..width {
        let coord = Coordinate {
          x: j as i32 - (width / 2) as i32,
          y: (height / 2) as i32 - i as i32,
        };

        match matrix[i][j] {
          '#' => status.insert(coord, InfectionStatus::Infected),
          'W' => status.insert(coord, InfectionStatus::Weakened),
          'F' => status.insert(coord, InfectionStatus::Flagged),
          _ => None,
        };
      }
    }

    Infection {
      status,
      position: Coordinate::origin(),
      direction: Direction::Up,
      count_infected: 0,
    }
  }

  fn next(&mut self) {
    if self.status.contains_key(&self.position) {
      // Uninfect and turn right
      self.status.remove(&self.position);
      self.direction = self.direction.right();
      self.position = self.position.mov(self.direction);
    } else {
      // Infect and turn left
      self.status.insert(self.position, InfectionStatus::Infected);
      self.direction = self.direction.left();
      self.position = self.position.mov(self.direction);
      self.count_infected += 1;
    }
  }

  fn strong_next(&mut self) {
    match self.status.get(&self.position) {
      Some(&InfectionStatus::Infected) => {
        // Flag and turn right
        self.status.insert(self.position, InfectionStatus::Flagged);
        self.direction = self.direction.right();
        self.position = self.position.mov(self.direction);
      }
      Some(&InfectionStatus::Weakened) => {
        // Infect
        self.status.insert(self.position, InfectionStatus::Infected);
        self.position = self.position.mov(self.direction);
        self.count_infected += 1;
      }
      Some(&InfectionStatus::Flagged) => {
        // Clean and reverse
        self.status.remove(&self.position);
        self.direction = self.direction.reverse();
        self.position = self.position.mov(self.direction);
      }
      None => {
        // Weaken and turn left
        self.status.insert(self.position, InfectionStatus::Weakened);
        self.direction = self.direction.left();
        self.position = self.position.mov(self.direction);
      }
    }
  }
}
