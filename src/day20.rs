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

  #[test]
  fn no2_sample_test() {
    let input = indoc!(
      "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
      p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
      p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
      p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>"
    );
    assert_eq!(1, run2(input));
  }

  #[test]
  fn no1_test() {
    assert_eq!(161, no1());
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

pub fn no2() -> usize {
  let input = include_str!("../inputs/input20").trim_right();
  run2(input)
}

fn run1(data: &str) -> usize {
  let re = Regex::new(
    r"^p=<(.*),(.*),(.*)>, v=<(.*),(.*),(.*)>, a=<(.*),(.*),(.*)>$",
  ).expect("Regex error");

  data
    .lines()
    .enumerate()
    .min_by_key(|&(_idx, l)| {
      for cap in re.captures_iter(l) {
        let x = Equation::form_abs_equation(&cap[1], &cap[4], &cap[7]);
        let y = Equation::form_abs_equation(&cap[2], &cap[5], &cap[8]);
        let z = Equation::form_abs_equation(&cap[3], &cap[6], &cap[9]);
        return Equation::sum(x, y, z);
      }
      panic!("No capture");
    })
    .expect("No min")
    .0
}

fn run2(data: &str) -> usize {
  let re = Regex::new(
    r"^p=<(.*),(.*),(.*)>, v=<(.*),(.*),(.*)>, a=<(.*),(.*),(.*)>$",
  ).expect("Regex error");
  let mut particles = data.lines();

  particles
    .clone()
    .filter(|&current| {
      particles.all(|other| {
        if current == other {
          return true;
        }

        for (cap_current, cap_other) in
          re.captures_iter(current).zip(re.captures_iter(other))
        {
          let current_x = Equation::form_equation(
            &cap_current[1],
            &cap_current[4],
            &cap_current[7],
          );
          let other_x = Equation::form_equation(
            &cap_other[1],
            &cap_other[4],
            &cap_other[7],
          );
          let current_y = Equation::form_equation(
            &cap_current[2],
            &cap_current[5],
            &cap_current[8],
          );
          let other_y = Equation::form_equation(
            &cap_other[2],
            &cap_other[5],
            &cap_other[8],
          );
          let current_z = Equation::form_equation(
            &cap_current[3],
            &cap_current[6],
            &cap_current[9],
          );
          let other_z = Equation::form_equation(
            &cap_other[3],
            &cap_other[6],
            &cap_other[9],
          );

          return !(current_x.collide(other_x) || current_y.collide(other_y) ||
                     current_z.collide(other_z));
        }

        panic!("No capture");
      })
    })
    .count()
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

    Equation {
      a: a,
      b: 2 * v + a,
      c: 2 * p,
    }
  }

  fn form_abs_equation(pos: &str, vel: &str, acc: &str) -> Equation {
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

  fn collide(&self, other: Equation) -> bool {
    let a = self.a - other.a;
    let b = self.b - other.b;
    let c = self.c - other.c;

    if a == 0 {
      if b == 0 {
        return c == 0;
      } else {
        // x = -c / b
        return c % b == 0 && b * c <= 0;
      }
    }

    let d_squared = b * b - 4 * a * c;
    if d_squared < 0 {
      return false; // no roots
    }
    let d = (d_squared as f32).sqrt() as i32;
    if d * d != d_squared {
      return false; // not exact square
    }

    let first_root_2a = -1 * b + d;
    let second_root_2a = -1 * b - d;
    first_root_2a * a >= 0 && second_root_2a * a >= 0 &&
      first_root_2a % (2 * a) == 0 && second_root_2a % (2 * a) == 0
  }
}
