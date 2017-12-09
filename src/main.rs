extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate indoc;

mod helper;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
//mod day07;
mod day08;
mod day09;

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
        (2, 1) => println!("{}", day02::no1()),
        (2, 2) => println!("{}", day02::no2()),
        (3, 1) => println!("{}", day03::no1()),
        (3, 2) => println!("{}", day03::no2()),
        (4, 1) => println!("{}", day04::no1()),
        (4, 2) => println!("{}", day04::no2()),
        (5, 1) => println!("{}", day05::no1()),
        (5, 2) => println!("{}", day05::no2()),
        (6, 1) => println!("{}", day06::no1()),
        (6, 2) => println!("{}", day06::no2()),
        // (7, 1) => println!("{}", day07::no1()),
        // (7, 2) => println!("{}", day07::no2()),
        (8, 1) => println!("{}", day08::no1()),
        (8, 2) => println!("{}", day08::no2()),
        (9, 1) => println!("{}", day09::no1()),
        (9, 2) => println!("{}", day09::no2()),
        _ => println!("Day {} number {} not found", day, no),
      }
    } else {
      println!("No is invalid");
      exit(1);
    }
  } else {
    println!("Day is invalid");
    exit(1);
  }
}
