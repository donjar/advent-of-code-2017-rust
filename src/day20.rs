use regex::Regex;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = indoc!(
      "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
      p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>"
    );
    assert_eq!(0, run1(input));
  }

  #[ignore]
  #[test]
  fn no2_sample_test() {
    let input = indoc!(
      "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
      p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>"
    );
    assert_eq!(0, run2(input));
  }

  #[test]
  fn no1_test() {
    assert_eq!(47, no1());
  }

  #[ignore]
  #[test]
  fn no2_test() {
    assert_eq!(213, no2());
  }
}

pub fn no1() -> usize {
  let input = include_str!("../inputs/input20").trim_right();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input20").trim_right();
  run2(input)
}

fn run1(data: &str) -> usize {
  let re = Regex::new(
    r"p=<(.*),(.*),(.*)>, v=<(.*),(.*),(.*)>, a=<(.*),(.*),(.*)>",
  ).expect("Regex error");

  data
    .lines()
    .enumerate()
    .min_by_key(|&(_idx, l)| {
      for cap in re.captures_iter(l) {
        let x = Equation::form_equation(&cap[1], &cap[4], &cap[7]);
        let y = Equation::form_equation(&cap[2], &cap[5], &cap[8]);
        let z = Equation::form_equation(&cap[3], &cap[6], &cap[9]);
        return Equation::sum(x, y, z);
      }
      panic!("No capture");
    })
    .expect("No min")
    .0
}

fn run2(data: &str) -> i32 {
  0
}

#[derive(Ord, PartialOrd, PartialEq, Eq)]
struct Equation {
  a: i32,
  b: i32,
  c: i32,
}

impl Equation {
  fn form_equation(pos: &str, vel: &str, acc: &str) -> Equation {
    let p: i32 = pos.parse().expect("Position parse error");
    let v: i32 = vel.parse().expect("Position parse error");
    let a: i32 = acc.parse().expect("Position parse error");

    if a > 0 {
      Equation {
        a: a,
        b: 2 * v + a,
        c: 2 * p,
      }
    } else {
      Equation {
        a: -1 * a,
        b: -1 * (2 * v + a),
        c: -2 * p,
      }
    }
  }

  fn sum(x: Equation, y: Equation, z: Equation) -> Equation {
    Equation {
      a: x.a + y.a + z.a,
      b: x.b + y.b + z.b,
      c: x.c + y.c + z.c,
    }
  }
}
