use helper::StringExt;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = "0\n3\n0\n1\n-3";
    assert_eq!(5, run(input, false));
  }

  #[test]
  fn no2_sample_test() {
    let input = "0\n3\n0\n1\n-3";
    assert_eq!(10, run(input, true));
  }

  #[test]
  fn no1_test() {
    assert_eq!(381680, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(29717847, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input5").trim();
  run(input, false)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input5").trim();
  run(input, true)
}

fn run(instructions: &str, type2: bool) -> i32 {
  let mut ins = instructions.split_into_data::<i32>();
  let mut pos: i32 = 0;
  let mut steps = 0;

  while 0 <= pos && pos < ins.len() as i32 {
    let offset = ins[pos as usize];

    if type2 && ins[pos as usize] >= 3 {
      ins[pos as usize] -= 1;
    } else {
      ins[pos as usize] += 1;
    }

    pos += offset;
    steps += 1;
  }

  steps
}
