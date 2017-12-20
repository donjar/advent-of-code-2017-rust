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

  #[test]
  fn no2_test() {
    assert_eq!(3833504, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input13").trim_right();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input13").trim_right();
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

        if hit(depth, range) {
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

  let all_divs: Vec<(_, _)> = record
    .lines()
    .map(|l| {
      for captures in re.captures_iter(l) {
        let depth: i32 = captures[1].parse().expect("Error unwrapping depth");
        let range: i32 = captures[2].parse().expect("Error unwrapping range");
        let modulo = 2 * (range - 1);
        return ((modulo - (depth % modulo)) % modulo, modulo);
      }
      panic!("Not captured");
    })
    .collect();

  let mut ans = 0;
  loop {
    let mut iter = all_divs.iter().cloned();
    if iter.all(|(remainder, modulo)| ans % modulo != remainder) {
      return ans;
    }
    ans += 1;
  }
}

fn hit(depth: i32, range: i32) -> bool {
  depth % (2 * (range - 1)) == 0
}
