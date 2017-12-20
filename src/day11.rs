use std::cmp::max;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    assert_eq!(3, run1("ne,ne,ne"));
    assert_eq!(0, run1("ne,ne,sw,sw"));
    assert_eq!(2, run1("ne,ne,s,s"));
    assert_eq!(3, run1("se,sw,se,sw,sw"));
  }

  #[test]
  fn no2_sample_test() {
    assert_eq!(3, run2("ne,ne,ne"));
    assert_eq!(2, run2("ne,ne,sw,sw"));
    assert_eq!(2, run2("ne,ne,s,s"));
    assert_eq!(3, run2("se,sw,se,sw,sw"));
  }

  #[test]
  fn no1_test() {
    assert_eq!(794, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(1524, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input11").trim_right();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input11").trim_right();
  run2(input)
}

fn run1(path: &str) -> i32 {
  let dirs = path.split(",");
  let mut x = 0;
  let mut y = 0;

  for dir in dirs {
    next(dir, &mut x, &mut y);
  }

  (x.abs() + max(x.abs(), y.abs())) / 2
}

fn run2(path: &str) -> i32 {
  let dirs = path.split(",");
  let mut x = 0;
  let mut y = 0;

  let mut maximum = 0;

  for dir in dirs {
    next(dir, &mut x, &mut y);
    maximum = max(maximum, (x.abs() + max(x.abs(), y.abs())) / 2);
  }
  maximum
}

fn next(dir: &str, x: &mut i32, y: &mut i32) {
  match dir {
    "n" => *y += 2,
    "ne" => {
      *x += 1;
      *y += 1
    }
    "se" => {
      *x += 1;
      *y -= 1
    }
    "s" => *y -= 2,
    "nw" => {
      *x -= 1;
      *y += 1
    }
    "sw" => {
      *x -= 1;
      *y -= 1
    }
    _ => panic!(format!("Found {}", dir)),
  }
}
