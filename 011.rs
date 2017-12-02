fn main() {
  let input = include_str!("input1");
  let trimmed_input = &input[..input.len() - 1];
  println!("{}", run(trimmed_input, 1));
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
