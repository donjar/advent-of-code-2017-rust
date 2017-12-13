use regex::Regex;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = indoc!(
      "0: 3
       1: 2
       4: 4
       6: 4"
    );
    assert_eq!(24, run1(input));
  }

  #[test]
  fn no2_sample_test() {
    let input = indoc!(
      "0: 3
       1: 2
       4: 4
       6: 4"
    );
    assert_eq!(10, run2(input));
  }

  #[test]
  fn no1_test() {
    assert_eq!(1904, no1());
  }

  #[ignore]
  #[test]
  fn no2_test() {
    assert_eq!(3833504, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input13").trim();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input13").trim();
  run2(input)
}

fn run1(record: &str) -> i32 {
  let re = Regex::new(r"^([[:digit:]]+): ([[:digit:]]+)$").unwrap();

  record
    .lines()
    .map(|l| {
      for captures in re.captures_iter(l) {
        let depth = captures[1].parse().expect("Error unwrapping depth");
        let range = captures[2].parse().expect("Error unwrapping range");

        if hit(depth, range, 0) {
          return depth * range;
        } else {
          return 0;
        }
      }
      panic!("Not captured");
    })
    .sum()
}

fn run2(record: &str) -> i32 {
  let re = Regex::new(r"^([[:digit:]]+): ([[:digit:]]+)$").unwrap();

  let mut ans = 0;
  let mut found = true;
  loop {
    for l in record.lines() {
      for captures in re.captures_iter(l) {
        let depth = captures[1].parse().expect("Error unwrapping depth");
        let range = captures[2].parse().expect("Error unwrapping range");

        if hit(depth, range, ans) {
          found = false;
          break;
        }
      }

      if !found {
        break;
      }
    }

    if found {
      return ans;
    } else {
      ans += 1;
      found = true;
    }
  }
}

fn hit(depth: i32, range: i32, offset: i32) -> bool {
  (depth + offset) % (2 * (range - 1)) == 0
}
