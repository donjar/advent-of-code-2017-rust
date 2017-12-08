pub fn no1() -> i32 {
  let input = include_str!("../inputs/input5").trim();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input5").trim();
  run2(input)
}

fn run1(instructions: &str) -> i32 {
  let mut ins = instructions
    .lines()
    .filter_map(|s| s.parse().ok())
    .collect::<Vec<i32>>();
  let mut pos: i32 = 0;
  let mut steps = 0;

  while 0 <= pos && pos < ins.len() as i32 {
    ins[pos as usize] += 1;
    pos += ins[pos as usize] - 1;
    steps += 1;
  }

  steps
}

fn run2(instructions: &str) -> i32 {
  let mut ins = instructions
    .lines()
    .filter_map(|s| s.parse().ok())
    .collect::<Vec<i32>>();
  let mut pos: i32 = 0;
  let mut steps = 0;

  while 0 <= pos && pos < ins.len() as i32 {
    let offset = ins[pos as usize];

    if ins[pos as usize] >= 3 {
      ins[pos as usize] -= 1;
    } else {
      ins[pos as usize] += 1;
    }

    pos += offset;
    steps += 1;
  }

  steps
}
