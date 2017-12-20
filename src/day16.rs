use std::collections::HashMap;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    assert_eq!("baedc", run1("s1,x3/4,pe/b", 5));
  }

  #[test]
  fn no2_sample_test() {
    assert_eq!("ceadb", run2("s1,x3/4,pe/b", 5, 2));
  }

  #[test]
  fn no1_test() {
    assert_eq!("fnloekigdmpajchb", no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!("amkjepdhifolgncb", no2());
  }
}

pub fn no1() -> String {
  let input = include_str!("../inputs/input16").trim_right();
  run1(input, 16)
}

pub fn no2() -> String {
  let input = include_str!("../inputs/input16").trim_right();
  run2(input, 16, 1_000_000_000)
}

fn run1(input: &str, programs: u8) -> String {
  let moves = input.split(',').collect();

  let mut line = Vec::new();
  for i in 0..programs {
    line.push(('a' as u8) + i);
  }

  dance(moves, &mut line);
  String::from_utf8(line).expect("String conversion fail")
}

fn run2(input: &str, programs: u8, amount: i32) -> String {
  let moves: Vec<&str> = input.split(',').collect();

  let mut line = Vec::new();
  for i in 0..programs {
    line.push(('a' as u8) + i);
  }

  let mut iteration_map: HashMap<i32, Vec<u8>> = HashMap::new();
  let mut line_map = HashMap::new();

  for i in 0..amount {
    if line_map.contains_key(&line) {
      line = iteration_map.get(&(amount % i)).unwrap().clone();
      break;
    }

    line_map.insert(line.clone(), i);
    iteration_map.insert(i, line.clone());

    dance(moves.clone(), &mut line);
  }
  String::from_utf8(line).expect("String conversion fail")
}

fn dance(moves: Vec<&str>, mut position: &mut Vec<u8>) {
  for m in moves {
    let ins = &m[0..1];
    let rest = &m[1..];

    match ins {
      "s" => {
        let amount = rest.parse().expect("Parse error");
        spin(&mut position, amount);
      }
      "x" => {
        let amounts: Vec<usize> = rest
          .split('/')
          .map(|i| i.parse().expect("Parse error"))
          .collect();
        exchange(&mut position, amounts[0], amounts[1]);
      }
      "p" => {
        let programs: Vec<&str> = rest.split('/').collect();
        partner(&mut position, programs[0], programs[1]);
      }
      _ => panic!(format!("Instruction {} not detected", ins)),
    }
  }

}

fn spin(line: &mut [u8], amount: usize) {
  let line_copy = line.to_vec();
  let len = line.len();

  for (idx, elem) in line.iter_mut().enumerate() {
    *elem = line_copy[(idx + len - amount) % len];
  }
}

fn exchange(line: &mut [u8], first: usize, second: usize) {
  line.swap(first, second)
}

fn partner(line: &mut [u8], first: &str, second: &str) {
  let first_u8 = first.as_bytes().first().unwrap();
  let second_u8 = second.as_bytes().first().unwrap();

  for elem in line.iter_mut() {
    if elem == first_u8 {
      *elem = second_u8.clone();
    } else if elem == second_u8 {
      *elem = first_u8.clone();
    }
  }
}
