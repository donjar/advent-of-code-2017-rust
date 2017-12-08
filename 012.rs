fn main() {
  let input = include_str!("input1").trim();
  println!("{}", run(input, input.len() / 2));
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
