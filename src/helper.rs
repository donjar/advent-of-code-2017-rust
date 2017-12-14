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
