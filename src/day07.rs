#[cfg(test)]
mod tests {
  use super::*;

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
  let input = include_str!("../inputs/input7").trim();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input7").trim();
  run2(input)
}
