use std::str::FromStr;

pub trait StringExt {
  fn split_into_data<T: FromStr>(&self) -> Vec<T>;
}

impl<'a> StringExt for &'a str {
  fn split_into_data<T: FromStr>(&self) -> Vec<T> {
    self
      .split_whitespace()
      .filter_map(|e| e.parse().ok())
      .collect::<Vec<T>>()
  }
}
