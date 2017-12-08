#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_test() {
    assert_eq!(21845, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(191, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input2").trim();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input2").trim();
  run2(input)
}

fn run1(spreadsheet: &str) -> i32 {
  let rows = spreadsheet.lines();

  rows
    .map(|row| {
      let data = row
        .split_whitespace()
        .filter_map(|e| e.parse().ok())
        .collect::<Vec<i32>>();
      let iter = data.iter();
      iter.clone().max().unwrap() - iter.min().unwrap()
    })
    .sum()
}

fn run2(spreadsheet: &str) -> i32 {
  let rows = spreadsheet.lines();

  rows
    .map(|row| {
      let data = row
        .split_whitespace()
        .filter_map(|e| e.parse().ok())
        .collect::<Vec<i32>>();
      let iter = data.iter();

      for x in iter.clone() {
        for y in iter.clone() {
          if x != y && x % y == 0 {
            return x / y;
          }
        }
      }
      0
    })
    .sum()
}
