fn main() {
  let input = include_str!("input2").trim();
  println!("{}", run(input));
}

fn run(spreadsheet: &str) -> i32 {
  let rows = spreadsheet.lines();

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
