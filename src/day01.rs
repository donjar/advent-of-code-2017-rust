#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    assert_eq!(3, run("1122", 1));
    assert_eq!(4, run("1111", 1));
    assert_eq!(0, run("1234", 1));
    assert_eq!(9, run("91212129", 1));
  }

  #[test]
  fn no2_sample_test() {
    assert_eq!(6, run("1212", 2));
    assert_eq!(0, run("1221", 2));
    assert_eq!(4, run("123425", 3));
    assert_eq!(12, run("123123", 3));
    assert_eq!(4, run("12131415", 4));
  }

  #[test]
  fn no1_test() {
    assert_eq!(1177, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(1060, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input1").trim();
  run(input, 1)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input1").trim();
  run(input, input.len() / 2)
}

fn run(captcha: &str, next_idx: usize) -> i32 {
  let bytes = captcha.as_bytes();

  bytes.iter().enumerate().fold(0, |s, (idx, byte)| {
    if byte == &bytes[(idx + next_idx) % bytes.len()] {
      return s + (byte - b'0') as i32;
    }
    s
  })
}
