use regex::Regex;
use std::collections::HashMap;

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

  #[test]
  fn no2_test() {
    assert_eq!(438, no2());
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
      let cap = re.captures(l).unwrap();
      let x = Equation::form_abs_equation(&cap[1], &cap[4], &cap[7]);
      let y = Equation::form_abs_equation(&cap[2], &cap[5], &cap[8]);
      let z = Equation::form_abs_equation(&cap[3], &cap[6], &cap[9]);

      let res = Equation::sum(x, y, z);
      res
    })
    .expect("No min")
    .0
}

fn run2(data: &str) -> usize {
  let re = Regex::new(
    r"^p=<(.*),(.*),(.*)>, v=<(.*),(.*),(.*)>, a=<(.*),(.*),(.*)>$",
  ).expect("Regex error");
  let mut particles: Vec<((i32, i32, i32), (i32, i32, i32), (i32, i32, i32))> =
    data.lines().map(|current| {
      let cap = re.captures(&current).unwrap();
      let x = (cap[1].parse().unwrap(),
               cap[4].parse().unwrap(),
               cap[7].parse().unwrap());
      let y = (cap[2].parse().unwrap(),
               cap[5].parse().unwrap(),
               cap[8].parse().unwrap());
      let z = (cap[3].parse().unwrap(),
               cap[6].parse().unwrap(),
               cap[9].parse().unwrap());
      (x, y, z)
  }).collect();

  for i in 0..1000 {
    // TODO fix this
    remove_collisions(&mut particles);
    advance(&mut particles);
  }

  particles.len()
}

fn remove_collisions(
  particles: &mut Vec<((i32, i32, i32), (i32, i32, i32), (i32, i32, i32))>,
) {
  let mut map = HashMap::new();

  for p in particles.iter().cloned() {
    *map.entry(((p.0).0, (p.1).0, (p.2).0)).or_insert(0) += 1;
  }

  let dups: Vec<(i32, i32, i32)> = map
    .iter()
    .filter(|&(_k, &v)| v > 1)
    .map(|(&k, _v)| k)
    .collect();

  let mut i = 0;
  while i < particles.len() {
    let p = particles[i];
    if dups.contains(&((p.0).0, (p.1).0, (p.2).0)) {
      particles.remove(i);
    } else {
      i += 1;
    }
  }
}

fn advance(
  particles: &mut Vec<((i32, i32, i32), (i32, i32, i32), (i32, i32, i32))>,
) {
  for p in particles.iter_mut() {
    (p.0).1 += (p.0).2;
    (p.1).1 += (p.1).2;
    (p.2).1 += (p.2).2;
    (p.0).0 += (p.0).1;
    (p.1).0 += (p.1).1;
    (p.2).0 += (p.2).1;
  }
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
struct Equation {
  a: i32,
  b: i32,
  c: i32,
}

impl Equation {
  fn form_abs_equation(pos: &str, vel: &str, acc: &str) -> Equation {
    let p: i32 = pos.parse().expect("Position parse error");
    let v: i32 = vel.parse().expect("Position parse error");
    let a: i32 = acc.parse().expect("Position parse error");

    if a > 0 || (a == 0 && v > 0) || (a == 0 && v == 0 && p > 0) {
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
