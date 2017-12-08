#[cfg(test)]
mod tests {
  use super::*;

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
