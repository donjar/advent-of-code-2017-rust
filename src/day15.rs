use regex::Regex;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    assert_eq!(588, run1(65, 8921));
  }

  #[test]
  fn no2_sample_test() {
    assert_eq!(309, run2(65, 8921));
  }

  #[test]
  fn no1_test() {
    assert_eq!(592, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(320, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input15").trim_right();
  let (a, b) = get_input(input);
  run1(a, b)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input15").trim_right();
  let (a, b) = get_input(input);
  run2(a, b)
}

fn get_input(input: &str) -> (i64, i64) {
  let re_a = Regex::new(r"^Generator A starts with ([[:digit:]]+)$").unwrap();
  let re_b = Regex::new(r"^Generator B starts with ([[:digit:]]+)$").unwrap();

  let mut lines = input.lines();
  let first_line = lines.next().unwrap();
  let second_line = lines.next().unwrap();

  let a = re_a.captures(first_line).unwrap()[1].parse().expect("Generator A input error");
  let b = re_b.captures(second_line).unwrap()[1].parse().expect("Generator B input error");
  (a, b)
}

fn run1(a_start: i64, b_start: i64) -> i32 {
  let mut a = a_start;
  let mut b = b_start;

  let mut sum = 0;
  for _ in 0..40_000_000 {
    a = (a * 16807) % 2147483647;
    b = (b * 48271) % 2147483647;
    if (a - b) % 65536 == 0 {
      sum += 1;
    }
  }
  sum
}

fn run2(a_start: i64, b_start: i64) -> i32 {
  let mut a = a_start;
  let mut b = b_start;

  let mut sum = 0;
  for _ in 0..5_000_000 {
    a = (a * 16807) % 2147483647;
    b = (b * 48271) % 2147483647;

    while a % 4 != 0 {
      a = (a * 16807) % 2147483647;
    }
    while b % 8 != 0 {
      b = (b * 48271) % 2147483647;
    }

    if (a - b) % 65536 == 0 {
      sum += 1;
    }
  }
  sum
}
