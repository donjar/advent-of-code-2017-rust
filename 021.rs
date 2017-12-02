fn main() {
  let input = include_str!("input2");
  let trimmed_input = &input[..input.len() - 1];

  println!("{}", run(trimmed_input));
}

fn run(spreadsheet: &str) -> i32 {
  let rows = spreadsheet.split("\n");

  rows
    .map(|row| {
      let data = row
        .split_whitespace()
        .filter_map(|e| e.parse().ok())
        .collect::<Vec<i32>>();
      let iter = data.iter();
      iter.clone().max().unwrap() - iter.min().unwrap()
    })
    .sum()
}
