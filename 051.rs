fn main() {
  let input = include_str!("input5");
  let trimmed_input = &input[..input.len() - 1]; // remove last newline

  println!("{}", run(trimmed_input));
}

fn run(instructions: &str) -> i32 {
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
