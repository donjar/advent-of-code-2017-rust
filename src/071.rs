extern crate regex;
use regex::Regex;

fn main() {
  let input = include_str!("input7");
  let trimmed_input = &input[..input.len() - 1]; // remove last newline

  println!("{}", run(trimmed_input));
}

fn run(programs: &str) -> &str {

}
