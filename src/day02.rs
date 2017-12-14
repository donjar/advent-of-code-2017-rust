use helper::DataSplitExt;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    assert_eq!(
      18,
      run1(indoc!(
        "5 1 9 5
         7 5 3
         2 4 6 8"
      ))
    );
  }

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
  let input = include_str!("../inputs/input02").trim();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input02").trim();
  run2(input)
}

fn run1(spreadsheet: &str) -> i32 {
  let rows = spreadsheet.lines();

  rows
    .map(|row| {
      let data = row.split_into_data::<i32>();
      let iter = data.iter();
      iter.clone().max().unwrap() - iter.min().unwrap()
    })
    .sum()
}

fn run2(spreadsheet: &str) -> i32 {
  let rows = spreadsheet.lines();

  rows
    .map(|row| {
      let data = row.split_into_data::<i32>();
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
