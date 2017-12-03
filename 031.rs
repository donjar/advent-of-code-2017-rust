fn main() {
  let input = 325489;
  println!("{}", run(input));
}

fn run(input: i32) -> i32 {
  if input == 1 {
    return 0;
  }

  let odd = odd_below_root(input - 1);
  let step = (odd + 1) / 2;
  let odd_sq = odd.pow(2);

  let increment = (input - odd_sq) / (2 * step);
  step + (input - odd_sq - (2 * increment + 1) * step).abs()

  // if input - odd_sq <= 2 * step { // e.g. 10, 11, 12, 13
  //   step + (input - odd_sq - step).abs()
  // } else if input - odd_sq <= 4 * step { // e.g. 14, 15, 16, 17
  //   step + (input - odd_sq - 3 * step).abs()
  // } else if input - odd_sq <= 6 * step { // e.g. 18, 19, 20, 21
  //   step + (input - odd_sq - 5 * step).abs()
  // } else { // e.g. 22, 23, 24, 25
  //   step + (input - odd_sq - 7 * step).abs()
  // }
}

fn odd_below_root(input: i32) -> i32 {
  (((input as f32).sqrt() + 1.0) / 2.0) as i32 * 2 - 1
}
