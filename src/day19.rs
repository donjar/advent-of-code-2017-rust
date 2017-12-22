use helper::{Coordinate, Direction};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[cfg_attr(rustfmt, rustfmt_skip)]
  fn sample_test() {
    let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+";
    assert_eq!((String::from("ABCDEF"), 38), run(input));
  }

  #[test]
  fn no1_test() {
    assert_eq!("XYFDJNRCQA", no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(17450, no2());
  }
}

pub fn no1() -> String {
  let input = include_str!("../inputs/input19").trim_right();
  run(input).0
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input19").trim_right();
  run(input).1
}

fn run(input: &str) -> (String, i32) {
  let matrix: Vec<Vec<char>> =
    input.lines().map(|l| l.chars().collect()).collect();
  let width = matrix[0].len();

  let mut current = (0, 0);
  for i in 0..width {
    if matrix[0][i] != ' ' {
      current = (0, i);
      break;
    }
  }

  let mut direction = Direction::Down;
  let mut letters = String::new();
  let mut count = 0;

  loop {
    let letter = matrix[current.0][current.1];
    match letter {
      '|' | '-' => {
        // continue
      }
      '+' => {
        // check for turns
        direction = Direction::all()
          .into_iter()
          .filter(|&d| {
            let c = next(current, d);
            d != direction.reverse() && matrix[c.0][c.1] != ' '
          })
          .next()
          .unwrap();
      }
      ' ' => {
        // stop
        return (letters, count);
      }
      _ => {
        // collect letter
        letters.push(letter);
      }
    }
    current = next(current, direction);
    count += 1;
  }
}

fn next(current: Coordinate<usize>, dir: Direction) -> Coordinate<usize> {
  match dir {
    Direction::Left => (current.x, current.y - 1),
    Direction::Up => (current.x - 1, current.y),
    Direction::Right => (current.x, current.y + 1),
    Direction::Down => (current.x + 1, current.y),
  }
}
