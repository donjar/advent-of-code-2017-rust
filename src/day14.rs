use helper::KnotHashExt;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = "flqrgnkx";
    assert_eq!(8108, run1(input));
  }

  #[test]
  fn no2_sample_test() {
    let input = "flqrgnkx";
    assert_eq!(1242, run2(input));
  }

  #[test]
  fn no1_test() {
    assert_eq!(8222, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(1086, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input14").trim();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input14").trim();
  run2(input)
}

fn run1(record: &str) -> i32 {
  (0..128)
    .map(|i| {
      let key: &str = &format!("{}-{}", record, i);
      key
        .knot_hash()
        .chars()
        .map(|c| c.to_digit(16).unwrap().count_ones() as i32)
        .sum::<i32>()
    })
    .sum()
}

fn run2(record: &str) -> i32 {
  let mut matrix: Vec<Vec<char>> = (0..128)
    .map(|i| {
      let key: &str = &format!("{}-{}", record, i);
      key
        .knot_hash()
        .chars()
        .flat_map(|c| {
          format!("{:04b}", c.to_digit(16).unwrap())
            .chars()
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
    })
    .collect();

  let mut count = 0;
  for i in 0..matrix.len() {
    for j in 0..matrix[i].len() {
      if matrix[i][j] == '1' {
        remove_component(&mut matrix, i, j);
        count += 1;
      }
    }
  }

  count
}

fn remove_component(matrix: &mut Vec<Vec<char>>, row: usize, col: usize) {
  let mut dfs_stack = Vec::new();

  dfs_stack.push((row, col));

  while !dfs_stack.is_empty() {
    let next = dfs_stack.pop().unwrap();
    let r = next.0;
    let c = next.1;
    matrix[r][c] = '0';

    if r > 0 && matrix[r - 1][c] == '1' {
      dfs_stack.push((r - 1, c));
    }
    if c > 0 && matrix[r][c - 1] == '1' {
      dfs_stack.push((r, c - 1));
    }
    if r < matrix.len() - 1 && matrix[r + 1][c] == '1' {
      dfs_stack.push((r + 1, c));
    }
    if c < matrix.len() - 1 && matrix[r][c + 1] == '1' {
      dfs_stack.push((r, c + 1));
    }
  }
}
