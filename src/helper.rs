pub trait StringExt {
  fn split_into_data(&self) -> Vec<i32>;
}

impl StringExt for String {
  fn split_into_data(&self) -> Vec<i32> {
    self.split_whitespace().filter_map(|e| e.parse().ok()).collect::<Vec<i32>>()
  }
}
