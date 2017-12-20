use noisy_float::prelude::r64;
use std::collections::BTreeMap;

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

  #[ignore]
  #[test]
  fn no2_test() {
    assert_eq!(10, no2());
  }
}

pub fn no1() -> usize {
  let input = include_str!("../inputs/input17").trim_right().parse().unwrap();
  run1(input)
}

pub fn no2() -> usize {
  let input = include_str!("../inputs/input17").trim_right().parse().unwrap();
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
  let mut tree = BTreeMap::new();
  tree.insert(r64(0.0), 0);

  for i in 1..(iterations + 1) {
    position += input;
    position %= i;

    let mut keys = tree.keys();
    if let Some(&key_before) = keys.nth(position) {
      if let Some(&key_after) = keys.next() {
        tree.insert((key_before + key_after) / 2.0, i);
      } else {
        tree.insert(key_before * 2.0, i);
      }
    }
  }

  let mut iter = tree.values();
  iter.find(|&i| i == &0);
  *iter.next().unwrap()
}
