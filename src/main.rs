mod day01;

use std::env;
use std::process::exit;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 3 {
    println!("Usage: {} day no", &args[0]);
    exit(1);
  }

  if let Some(day) = args[1].parse::<i32>().ok() {
    if let Some(no) = args[2].parse::<i32>().ok() {
      match (day, no) {
        (1, 1) => println!("{}", day01::no1()),
        (1, 2) => println!("{}", day01::no2()),
        _ => println!("Day {} number {} not found", day, no),
      }
    } else {
      println!("No is invalid");
    }
  } else {
    println!("Day is invalid");
  }
}
