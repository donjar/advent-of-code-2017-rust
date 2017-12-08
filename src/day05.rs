#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_test() {
    assert_eq!(381680, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(29717847, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input5").trim();
  run(input, false)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input5").trim();
  run(input, true)
}

fn run(instructions: &str, type2: bool) -> i32 {
  let mut ins = instructions
    .lines()
    .filter_map(|s| s.parse().ok())
    .collect::<Vec<i32>>();
  let mut pos: i32 = 0;
  let mut steps = 0;

  while 0 <= pos && pos < ins.len() as i32 {
    let offset = ins[pos as usize];

    if type2 && ins[pos as usize] >= 3 {
      ins[pos as usize] -= 1;
    } else {
      ins[pos as usize] += 1;
    }

    pos += offset;
    steps += 1;
  }

  steps
}
