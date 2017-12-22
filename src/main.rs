extern crate regex;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate indoc;
extern crate num;

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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
//mod day23;
//mod day24;
//mod day25;

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
        (11, 1) => println!("{}", day11::no1()),
        (11, 2) => println!("{}", day11::no2()),
        (12, 1) => println!("{}", day12::no1()),
        (12, 2) => println!("{}", day12::no2()),
        (13, 1) => println!("{}", day13::no1()),
        (13, 2) => println!("{}", day13::no2()),
        (14, 1) => println!("{}", day14::no1()),
        (14, 2) => println!("{}", day14::no2()),
        (15, 1) => println!("{}", day15::no1()),
        (15, 2) => println!("{}", day15::no2()),
        (16, 1) => println!("{}", day16::no1()),
        (16, 2) => println!("{}", day16::no2()),
        (17, 1) => println!("{}", day17::no1()),
        (17, 2) => println!("{}", day17::no2()),
        (18, 1) => println!("{}", day18::no1()),
        (18, 2) => println!("{}", day18::no2()),
        (19, 1) => println!("{}", day19::no1()),
        (19, 2) => println!("{}", day19::no2()),
        (20, 1) => println!("{}", day20::no1()),
        (20, 2) => println!("{}", day20::no2()),
        (21, 1) => println!("{}", day21::no1()),
        (21, 2) => println!("{}", day21::no2()),
        (22, 1) => println!("{}", day22::no1()),
        (22, 2) => println!("{}", day22::no2()),
        //(23, 1) => println!("{}", day23::no1()),
        //(23, 2) => println!("{}", day23::no2()),
        //(24, 1) => println!("{}", day24::no1()),
        //(24, 2) => println!("{}", day24::no2()),
        //(25, 1) => println!("{}", day25::no1()),
        //(25, 2) => println!("{}", day25::no2()),
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
