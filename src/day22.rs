use helper::{Coordinate, Direction};
use std::collections::HashSet;

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
    assert_eq!(24, run2(input));
  }

  #[test]
  fn no1_test() {
    assert_eq!(5460, no1());
  }

  #[ignore]
  #[test]
  fn no2_test() {
    assert_eq!(3833504, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input22").trim_right();
  run1(input, 10000)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input22").trim_right();
  run2(input)
}

fn run1(map: &str, iterations: i32) -> i32 {
  let mut i = Infection::new(map);
  for _ in 0..iterations {
    i.next();
  }
  i.count_infected
}

fn run2(map: &str) -> i32 {
  0
}

struct Infection {
  infected: HashSet<Coordinate<i32>>,
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

    let mut infected = HashSet::new();
    for i in 0..height {
      for j in 0..width {
        if matrix[i][j] == '#' {
          infected.insert(Coordinate {
            x: j as i32 - (width / 2) as i32,
            y: (height / 2) as i32 - i as i32,
          });
        }
      }
    }

    Infection {
      infected,
      position: Coordinate::origin(),
      direction: Direction::Up,
      count_infected: 0,
    }
  }

  fn next(&mut self) {
    if self.infected.contains(&self.position) {
      // Uninfect and turn right
      self.infected.remove(&self.position);
      self.direction = self.direction.right();
      self.position = self.position.mov(self.direction);
    } else {
      // Infect and turn left
      self.infected.insert(self.position);
      self.direction = self.direction.left();
      self.position = self.position.mov(self.direction);
      self.count_infected += 1;
    }
  }
}
