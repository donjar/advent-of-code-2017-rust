use num::PrimInt;
use std::str::FromStr;

pub trait DataSplitExt {
  fn split_into_data<T: FromStr>(&self) -> Vec<T>;
}

pub trait KnotHashExt {
  fn knot_hash(&self) -> String;
}

impl<'a> DataSplitExt for &'a str {
  fn split_into_data<T: FromStr>(&self) -> Vec<T> {
    self
      .split_whitespace()
      .filter_map(|e| e.parse().ok())
      .collect::<Vec<T>>()
  }
}

impl<'a> KnotHashExt for &'a str {
  fn knot_hash(&self) -> String {
    let lens = [self.as_bytes(), &[17, 31, 73, 47, 23]].concat();
    let usize_lens: Vec<usize> = lens.iter().map(|&i| i as usize).collect();

    let mut vec: Vec<i32> = (0..256).collect();
    let mut pointer = 0;
    let mut skip = 0;

    for _ in 0..64 {
      knot(
        &mut vec,
        usize_lens.iter().cloned(),
        &mut pointer,
        &mut skip,
      );
    }

    vec.chunks(16).fold(String::new(), |text, chunk| {
      text + &format!("{:02x}", chunk.iter().fold(0, |acc, &x| acc ^ x))
    })
  }
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate<T> {
  pub x: T,
  pub y: T,
}

impl<T> Coordinate<T> where T: PrimInt {
  pub fn origin() -> Coordinate<T> {
    Coordinate { x: T::zero(), y: T::zero() }
  }

  pub fn day03_next(&self) -> Coordinate<T> {
    let minus_one = T::zero() - T::one();

    if self.y <= T::zero() && self.y <= self.x && self.x <= minus_one * self.y {
      // (-2, -2), (-1, -2), (0, -2), (1, -2), (2, -2): go right
      Coordinate { x: self.x + T::one(), y: self.y }
    } else if self.x > T::zero() && minus_one * self.x < self.y && self.y < self.x {
      // (2, -1), (2, 0), (2, 1): go up
      Coordinate { x: self.x, y: self.y + T::one() }
    } else if self.y > T::zero() && minus_one * self.y < self.x && self.x <= self.y {
      // (2, 2), (1, 2), (0, 2), (-1, 2): go left
      Coordinate { x: self.x - T::one(), y: self.y }
    } else {
      // go down
      Coordinate { x: self.x, y: self.y - T::one() }
    }
  }

  pub fn neighbors(&self) -> [Coordinate<T>; 8] {
    [
      Coordinate { x: self.x + T::one(), y: self.y + T::one() },
      Coordinate { x: self.x, y: self.y + T::one() },
      Coordinate { x: self.x - T::one(), y: self.y + T::one() },
      Coordinate { x: self.x + T::one(), y: self.y },
      Coordinate { x: self.x - T::one(), y: self.y },
      Coordinate { x: self.x + T::one(), y: self.y - T::one() },
      Coordinate { x: self.x, y: self.y - T::one() },
      Coordinate { x: self.x - T::one(), y: self.y - T::one() },
    ]
  }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
  Left,
  Up,
  Right,
  Down,
}

impl Direction {
  pub fn all() -> Vec<Direction> {
    vec![
      Direction::Left,
      Direction::Up,
      Direction::Right,
      Direction::Down,
    ]
  }

  pub fn reverse(self) -> Direction {
    match self {
      Direction::Left => Direction::Right,
      Direction::Up => Direction::Down,
      Direction::Right => Direction::Left,
      Direction::Down => Direction::Up,
    }
  }
}

