#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    assert_eq!(638, run1(3));
  }

  #[test]
  fn no2_sample_test() {
    assert_eq!(9, run2(3, 9));
  }

  #[test]
  fn no1_test() {
    assert_eq!(725, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(27361412, no2());
  }
}

pub fn no1() -> usize {
  let input = include_str!("../inputs/input17")
    .trim_right()
    .parse()
    .unwrap();
  run1(input)
}

pub fn no2() -> usize {
  let input = include_str!("../inputs/input17")
    .trim_right()
    .parse()
    .unwrap();
  run2(input, 50_000_000)
}

fn run1(input: usize) -> usize {
  let mut position = 0;
  let mut vec = Vec::new();
  vec.push(0);

  for i in 1..2018 {
    position += input;
    position %= i;

    position += 1;
    vec.insert(position, i);
  }

  vec[position + 1]
}

fn run2(input: usize, iterations: usize) -> usize {
  let mut position = 0;
  let mut int_after = 0;

  for i in 1..(iterations + 1) {
    position += input;
    position %= i;
    if position == 0 {
      int_after = i;
    }
    position += 1;
  }
  int_after
}
