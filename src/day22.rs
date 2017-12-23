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

  #[ignore]
  #[test]
  fn no1_test() {
    assert_eq!(1904, no1());
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
  0
}

fn run2(map: &str) -> i32 {
  0
}

struct Infection {
  infected: HashSet<Coordinate<i32>>,
  position: Coordinate<i32>,
  direction: Direction,
}

impl Infection {
  fn new(map: &str) {
    let matrix: Vec<Vec<char>> = map.lines().map(|l| l.chars().collect()).collect();
    let height = matrix.len();
    let width = matrix[0].len();
  }
}
