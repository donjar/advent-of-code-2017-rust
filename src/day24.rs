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

  #[test]
  fn no2_sample_test() {
    assert_eq!(
      19,
      run2(indoc!(
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
  fn no1_test() {
    assert_eq!(1940, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(1928, no2());
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

fn run2(ports: &str) -> i32 {
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
  get_max_connection_with_len(p, 0).1
}

fn get_max_connection(ports: Vec<(i32, i32)>, start: i32) -> i32 {
  ports
    .iter()
    .filter_map(|p| {
      if p.0 != start && p.1 != start {
        return None;
      }

      let pin_sum = p.0 + p.1;
      let other = pin_sum - start;
      Some(
        pin_sum +
          get_max_connection(
            ports.iter().cloned().filter(|port| port != p).collect(),
            other,
          ),
      )
    })
    .max()
    .unwrap_or(0)
}

fn get_max_connection_with_len(
  ports: Vec<(i32, i32)>,
  start: i32,
) -> (i32, i32) {
  ports
    .iter()
    .filter_map(|p| {
      if p.0 != start && p.1 != start {
        return None;
      }

      let pin_sum = p.0 + p.1;
      let other = pin_sum - start;
      let other_pins = ports.iter().cloned().filter(|port| port != p).collect();
      let rest = get_max_connection_with_len(other_pins, other);

      let res = Some((1 + rest.0, pin_sum + rest.1));
      res
    })
    .max()
    .unwrap_or((0, 0))
}
