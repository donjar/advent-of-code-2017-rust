use std::collections::HashMap;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    assert_eq!(0, run1(1));
    assert_eq!(3, run1(12));
    assert_eq!(2, run1(23));
    assert_eq!(31, run1(1024));
  }

  #[test]
  fn no2_sample_test() {
    assert_eq!(1, run2(0));
    assert_eq!(23, run2(12));
    assert_eq!(25, run2(24));
    assert_eq!(747, run2(380));
  }

  #[test]
  fn no1_test() {
    assert_eq!(552, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(330785, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input03").trim().parse().unwrap();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input03").trim().parse().unwrap();
  run2(input)
}

fn run1(input: i32) -> i32 {
  if input == 1 {
    return 0;
  }

  let odd = odd_below_root(input - 1);
  let step = (odd + 1) / 2;
  let odd_sq = odd.pow(2);

  let increment = (input - odd_sq) / (2 * step);
  step + (input - odd_sq - (2 * increment + 1) * step).abs()

  // if input - odd_sq <= 2 * step { // e.g. 10, 11, 12, 13
  //   step + (input - odd_sq - step).abs()
  // } else if input - odd_sq <= 4 * step { // e.g. 14, 15, 16, 17
  //   step + (input - odd_sq - 3 * step).abs()
  // } else if input - odd_sq <= 6 * step { // e.g. 18, 19, 20, 21
  //   step + (input - odd_sq - 5 * step).abs()
  // } else { // e.g. 22, 23, 24, 25
  //   step + (input - odd_sq - 7 * step).abs()
  // }
}

fn run2(input: i32) -> i32 {
  let mut coordinate_map = HashMap::new();

  let mut current_coordinate = Coordinate(0, 0);
  let mut current_value = 1;

  while current_value < input {
    coordinate_map.insert(current_coordinate, current_value);

    current_coordinate = current_coordinate.next();
    current_value = current_coordinate
      .neighbors()
      .iter()
      .map(|c| coordinate_map.get(c).unwrap_or(&0))
      .sum()
  }

  current_value
}

fn odd_below_root(input: i32) -> i32 {
  (((input as f32).sqrt() + 1.0) / 2.0) as i32 * 2 - 1
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate(i32, i32);

impl Coordinate {
  fn next(&self) -> Coordinate {
    if self.1 <= 0 && self.1 <= self.0 && self.0 <= -1 * self.1 {
      // (-2, -2), (-1, -2), (0, -2), (1, -2), (2, -2): go right
      Coordinate(self.0 + 1, self.1)
    } else if self.0 > 0 && -1 * self.0 < self.1 && self.1 < self.0 {
      // (2, -1), (2, 0), (2, 1): go up
      Coordinate(self.0, self.1 + 1)
    } else if self.1 > 0 && -1 * self.1 < self.0 && self.0 <= self.1 {
      // (2, 2), (1, 2), (0, 2), (-1, 2): go left
      Coordinate(self.0 - 1, self.1)
    } else {
      // go down
      Coordinate(self.0, self.1 - 1)
    }
  }

  fn neighbors(&self) -> [Coordinate; 8] {
    [
      Coordinate(self.0 + 1, self.1 + 1),
      Coordinate(self.0, self.1 + 1),
      Coordinate(self.0 - 1, self.1 + 1),
      Coordinate(self.0 + 1, self.1),
      Coordinate(self.0 - 1, self.1),
      Coordinate(self.0 + 1, self.1 - 1),
      Coordinate(self.0, self.1 - 1),
      Coordinate(self.0 - 1, self.1 - 1),
    ]
  }
}
