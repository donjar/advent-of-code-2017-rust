use std::collections::HashMap;

fn main() {
  let input = include_str!("input6").trim();
  println!("{}", run(input));
}

fn run(blocks: &str) -> i32 {
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
