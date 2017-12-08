use std::collections::HashSet;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_test() {
    assert_eq!(477, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(167, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input4").trim();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input4").trim();
  run2(input)
}

fn run1(passphrases: &str) -> i32 {
  let lines = passphrases.lines();

  lines
    .map(|passphrase| {
      let mut words = HashSet::new();

      for w in passphrase.split(" ") {
        if words.contains(w) {
          return 0;
        }
        words.insert(w);
      }

      return 1;
    })
    .sum()
}

fn run2(passphrases: &str) -> i32 {
  let lines = passphrases.lines();

  lines
    .map(|passphrase| {
      let mut words = HashSet::new();

      for w in passphrase.split(" ") {
        let mut sorted = w.chars().collect::<Vec<char>>();
        sorted.sort();

        if words.contains(&sorted) {
          return 0;
        }
        words.insert(sorted);
      }

      return 1;
    })
    .sum()
}
