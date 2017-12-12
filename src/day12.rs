use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = indoc!(
      "0 <-> 2
       1 <-> 1
       2 <-> 0, 3, 4
       3 <-> 2, 4
       4 <-> 2, 3, 6
       5 <-> 6
       6 <-> 4, 5"
    );
    assert_eq!(6, run1(input));
  }

  #[test]
  fn no2_sample_test() {
    let input = indoc!(
      "0 <-> 2
       1 <-> 1
       2 <-> 0, 3, 4
       3 <-> 2, 4
       4 <-> 2, 3, 6
       5 <-> 6
       6 <-> 4, 5"
    );
    assert_eq!(2, run2(input));
  }

  #[test]
  fn no1_test() {
    assert_eq!(175, no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(213, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input12").trim();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input12").trim();
  run2(input)
}

fn run1(adj_list: &str) -> i32 {
  let adj_list_mapping = construct_adj_list(adj_list);

  // DFS
  let mut dfs_stack = Vec::new();
  let mut visited = HashSet::new();
  dfs_stack.push(String::from("0"));

  while !dfs_stack.is_empty() {
    let next = dfs_stack.pop().unwrap();

    for neighbor in adj_list_mapping.get(&next).unwrap() {
      if !visited.contains(neighbor) {
        dfs_stack.push(neighbor.clone());
      }
    }

    visited.insert(next);
  }

  visited.len() as i32
}

fn run2(adj_list: &str) -> i32 {
  let adj_list_mapping = construct_adj_list(adj_list);

  // DFS
  let mut total_cc = 0;

  let mut dfs_stack = Vec::new();
  let mut unvisited = HashSet::new();
  let mut visited = HashSet::new();

  for k in adj_list_mapping.keys() {
    unvisited.insert(k.clone());
  }

  while visited.len() != adj_list_mapping.len() {
    dfs_stack.push(unvisited.iter().next().unwrap().clone());
    total_cc += 1;

    while !dfs_stack.is_empty() {
      let next = dfs_stack.pop().unwrap();

      for neighbor in adj_list_mapping.get(&next).unwrap() {
        if !visited.contains(neighbor) {
          dfs_stack.push(neighbor.clone());
        }
      }

      visited.insert(next.clone());
      unvisited.remove(&next);
    }
  }

  total_cc
}

fn construct_adj_list(
  adj_list_string: &str,
) -> HashMap<String, HashSet<String>> {
  let adj_list_regex = Regex::new(r"^(.*) <-> (.*)$").unwrap();
  // Construct hashmap
  let mut h = HashMap::new();

  for l in adj_list_string.lines() {
    for captures in adj_list_regex.captures_iter(l) {
      let node = captures[1].to_string();
      let neighbors = captures[2].split(", ");

      let mut set = HashSet::new();
      for n in neighbors {
        set.insert(n.to_string());
      }
      h.insert(node, set);
    }
  }

  h
}
