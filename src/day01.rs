pub fn no1() -> u32 {
  let input = include_str!("../inputs/input1").trim();
  run(input, 1)
}

pub fn no2() -> u32 {
  let input = include_str!("../inputs/input1").trim();
  run(input, input.len() / 2)
}

fn run(captcha: &str, next_idx: usize) -> u32 {
  let bytes = captcha.as_bytes();

  bytes.iter().enumerate().fold(0, |s, (idx, byte)| {
    if byte == &bytes[(idx + next_idx) % bytes.len()] {
      return s + (byte - b'0') as u32;
    }
    s
  })
}
