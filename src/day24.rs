#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    assert_eq!(
      31,
      run1(indoc!(
        "0/2
         2/2
         2/3
         3/4
         3/5
         0/1
         10/1
         9/10"
      ))
    );
  }

  #[ignore]
  #[test]
  fn no2_sample_test() {
    assert_eq!(
      9,
      run2(indoc!(
        "5 9 2 8
         9 4 7 3
         3 8 6 5"
      ))
    );
  }

  #[ignore]
  #[test]
  fn no1_test() {
    assert_eq!(21845, no1());
  }

  #[ignore]
  #[test]
  fn no2_test() {
    assert_eq!(191, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input24").trim_right();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input24").trim_right();
  run2(input)
}

fn run1(ports: &str) -> i32 {
  let p = ports
    .lines()
    .map(|l| {
      let mut sp = l.split("/");
      (
        sp.next().expect("Should have two ports").parse().expect(
          "Pins should be integer",
        ),
        sp.next().expect("Should have two ports").parse().expect(
          "Pins should be integer",
        ),
      )
    })
    .collect();
  get_max_connection(p, 0)
}

fn run2(input: &str) -> i32 {
  0
}

fn get_max_connection(ports: Vec<(i32, i32)>, start: i32) -> i32 {
  ports
    .iter()
    .filter_map(|p| {
      if p.0 != start && p.1 != start {
        return None;
      }

      let other = p.0 + p.1 - start;
      Some(
        p.0 + p.1 +
          get_max_connection(
            ports.iter().cloned().filter(|port| port != p).collect(),
            other,
          ),
      )
    })
    .max()
    .unwrap_or(0)
}
