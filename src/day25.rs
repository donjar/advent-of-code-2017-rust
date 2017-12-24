#[cfg(test)]
mod tests {
  use super::*;

  #[ignore]
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

  #[ignore]
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

  #[ignore]
  #[test]
  fn no1_test() {
    assert_eq!(21845, no1());
  }

  #[ignore]
  #[test]
  fn no2_test() {
    assert_eq!(191, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input25").trim_right();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input25").trim_right();
  run2(input)
}

fn run1(input: &str) -> i32 {
  0
}

fn run2(input: &str) -> i32 {
  0
}
