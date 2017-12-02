use std::fs::File;
use std::io::Read;

fn main() {
  let input_file = "input2";

  let mut file = File::open(input_file).expect("File not found");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("File read error");
  contents.pop(); // remove last char newline

  println!("{}", run(&contents));
}

fn run(spreadsheet: &String) -> i32 {
  let rows = spreadsheet.split("\n");

  rows
    .map(|row| {
      let data = row
        .split_whitespace()
        .filter_map(|e| e.parse().ok())
        .collect::<Vec<i32>>();
      let iter = data.iter();

      for x in iter.clone() {
        for y in iter.clone() {
          if x != y && x % y == 0 {
            return x / y;
          }
        }
      }
      0
    })
    .sum()
}
