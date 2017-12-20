#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    assert_eq!(1, run1("{}"));
    assert_eq!(6, run1("{{{}}}"));
    assert_eq!(5, run1("{{},{}}"));
    assert_eq!(16, run1("{{{},{},{{}}}}"));
    assert_eq!(1, run1("{<a>,<a>,<a>,<a>}"));
    assert_eq!(9, run1("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    assert_eq!(9, run1("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    assert_eq!(3, run1("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
  }

  #[test]
  fn no2_sample_test() {
    assert_eq!(0, run2("<>"));
    assert_eq!(17, run2("<random characters>"));
    assert_eq!(3, run2("<<<<>"));
    assert_eq!(2, run2("<{!>}>"));
    assert_eq!(0, run2("<!!>"));
    assert_eq!(0, run2("<!!!>>"));
    assert_eq!(10, run2("<{o\"i!a,<{i<a>"));
  }

  #[test]
  fn no1_test() {
    assert_eq!(17537, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(7539, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input09").trim_right();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input09").trim_right();
  run2(input)
}

fn run1(stream: &str) -> i32 {
  let mut score = 0;
  let mut level = 0;
  let mut is_garbage = false;

  let mut i = 0;
  while i < stream.len() {
    match stream.as_bytes()[i] as char {
      '{' if !is_garbage => level += 1,
      '}' if !is_garbage => {
        score += level;
        level -= 1;
      }
      '<' => is_garbage = true,
      '>' => is_garbage = false,
      '!' => i += 1,
      _ => (),
    }
    i += 1;
  }

  score
}

fn run2(stream: &str) -> i32 {
  let mut is_garbage = false;
  let mut count = 0;

  let mut i = 0;
  while i < stream.len() {
    match stream.as_bytes()[i] as char {
      '<' => {
        if is_garbage {
          count += 1;
        } else {
          is_garbage = true;
        }
      }
      '>' => is_garbage = false,
      '!' => i += 1,
      _ if is_garbage => count += 1,
      _ => (),
    }
    i += 1;
  }

  count
}
