extern crate regex;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate indoc;

mod helper;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

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
        (01, 1) => println!("{}", day01::no1()),
        (01, 2) => println!("{}", day01::no2()),
        (02, 1) => println!("{}", day02::no1()),
        (02, 2) => println!("{}", day02::no2()),
        (03, 1) => println!("{}", day03::no1()),
        (03, 2) => println!("{}", day03::no2()),
        (04, 1) => println!("{}", day04::no1()),
        (04, 2) => println!("{}", day04::no2()),
        (05, 1) => println!("{}", day05::no1()),
        (05, 2) => println!("{}", day05::no2()),
        (06, 1) => println!("{}", day06::no1()),
        (06, 2) => println!("{}", day06::no2()),
        (07, 1) => println!("{}", day07::no1()),
        (07, 2) => println!("{}", day07::no2()),
        (08, 1) => println!("{}", day08::no1()),
        (08, 2) => println!("{}", day08::no2()),
        (09, 1) => println!("{}", day09::no1()),
        (09, 2) => println!("{}", day09::no2()),
        (10, 1) => println!("{}", day10::no1()),
        (10, 2) => println!("{}", day10::no2()),
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
