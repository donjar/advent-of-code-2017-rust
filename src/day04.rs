use std::collections::HashSet;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = indoc!(
      "aa bb cc dd ee
       aa bb cc dd aa
       aa bb cc dd aaa"
    );
    assert_eq!(2, run1(input));
  }

  #[test]
  fn no2_sample_test() {
    let input = indoc!(
      "abcde fghij
       abcde xyz ecdab
       a ab abc abd abf abj
       iiii oiii ooii oooi oooo
       oiii ioii iioi iiio"
    );
    assert_eq!(3, run2(input));
  }

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
  let input = include_str!("../inputs/input04").trim();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input04").trim();
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
        let mut sorted: Vec<char> = w.chars().collect();
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
