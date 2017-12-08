use std::collections::HashMap;

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input6").trim();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input6").trim();
  run2(input)
}

fn run1(blocks: &str) -> i32 {
  let mut pos = blocks
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect::<Vec<i32>>();
  let mut step = 0;
  let mut collection = HashMap::new();

  loop {
    if collection.get(&pos).cloned().is_some() {
      return step;
    } else {
      collection.insert(pos.clone(), step);
      step += 1;
      distribute(&mut pos);
    }
  }
}

fn run2(blocks: &str) -> i32 {
  let mut pos = blocks
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect::<Vec<i32>>();
  let mut step = 0;
  let mut collection = HashMap::new();

  loop {
    if let Some(s) = collection.get(&pos).cloned() {
      return step - s;
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
