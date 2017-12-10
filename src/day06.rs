use helper::StringExt;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = "0 2 7 0";
    assert_eq!(5, run(input, true));
  }

  #[test]
  fn no2_sample_test() {
    let input = "0 2 7 0";
    assert_eq!(4, run(input, false));
  }

  #[test]
  fn no1_test() {
    assert_eq!(5042, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(1086, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input06").trim();
  run(input, true)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input06").trim();
  run(input, false)
}

fn run(blocks: &str, find_position: bool) -> i32 {
  let mut pos = blocks.split_into_data::<i32>();
  let mut step = 0;
  let mut collection = HashMap::new();

  loop {
    if let Some(s) = collection.get(&pos).cloned() {
      if find_position {
        return step;
      } else {
        return step - s;
      }
    } else {
      collection.insert(pos.clone(), step);
      step += 1;
      distribute(&mut pos);
    }
  }
}

fn distribute(position: &mut [i32]) {
  let mut index = 0;
  let mut max = position[0];
  for (idx, &num) in position.iter().enumerate() {
    if num > max {
      index = idx;
      max = num;
    }
  }

  position[index] = 0;

  for i in 1..(max + 1) {
    let curr = (index + i as usize) % position.len();
    position[curr] += 1;
  }
}
