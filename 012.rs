use std::fs::File;
use std::io::Read;

fn main() {
  let input_file = "input1";

  let mut file = File::open(input_file).expect("File not found");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("File read error");
  contents.pop(); // remove last char newline

  println!("{}", run(&contents, &contents.len() / 2));
}

fn run(captcha: &String, next_idx: usize) -> u32 {
  let bytes = captcha.as_bytes();

  bytes.iter().enumerate().fold(0, |s, (idx, byte)| {
    if byte == &bytes[(idx + next_idx) % bytes.len()] {
      return s + (byte - b'0') as u32;
    }
    s
  })
}
