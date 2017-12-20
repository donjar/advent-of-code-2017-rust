use helper::KnotHashExt;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    assert_eq!(12, run1(5, "3,4,1,5"))
  }

  #[test]
  fn no2_sample_test() {
    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", run2(""));
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", run2("AoC 2017"));
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", run2("1,2,3"));
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", run2("1,2,4"));
  }

  #[test]
  fn no1_test() {
    assert_eq!(52070, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!("7f94112db4e32e19cf6502073c66f9bb", no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input10").trim_right();
  run1(256, input)
}

pub fn no2() -> String {
  let input = include_str!("../inputs/input10").trim_right();
  run2(input)
}

fn run1(list_size: i32, lengths: &str) -> i32 {
  let mut v: Vec<i32> = (0..list_size).collect();
  let lens = lengths.split(",").filter_map(|i| i.parse().ok());
  knot(&mut v, lens, &mut 0, &mut 0);
  v[0] * v[1]
}

fn run2(lengths: &str) -> String {
  lengths.knot_hash()
}

fn knot<'a, I>(
  mut vector: &mut [i32],
  lengths: I,
  pointer: &mut usize,
  skip: &mut usize,
) where
  I: Iterator<Item = usize>,
{
  for length in lengths {
    rev(&mut vector, *pointer, length);
    *pointer += length + *skip;
    *skip += 1;
  }
}

fn rev(vector: &mut [i32], start: usize, length: usize) {
  for i in 0..(length / 2) {
    let first = (start + i) % vector.len();
    let second = (length + start - 1 - i) % vector.len();
    vector.swap(first, second);
  }
}
