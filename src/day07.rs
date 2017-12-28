use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Sub;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = indoc!(
      "pbga (66)
       xhth (57)
       ebii (61)
       havc (66)
       ktlj (57)
       fwft (72) -> ktlj, cntj, xhth
       qoyq (66)
       padx (45) -> pbga, havc, qoyq
       tknk (41) -> ugml, padx, fwft
       jptl (61)
       ugml (68) -> gyxo, ebii, jptl
       gyxo (61)
       cntj (57)"
    );
    assert_eq!("tknk", run1(input));
  }

  #[test]
  fn no2_sample_test() {
    let input = indoc!(
      "pbga (66)
       xhth (57)
       ebii (61)
       havc (66)
       ktlj (57)
       fwft (72) -> ktlj, cntj, xhth
       qoyq (66)
       padx (45) -> pbga, havc, qoyq
       tknk (41) -> ugml, padx, fwft
       jptl (61)
       ugml (68) -> gyxo, ebii, jptl
       gyxo (61)
       cntj (57)"
    );
    assert_eq!(60, run2(input));
  }

  #[test]
  fn no1_test() {
    assert_eq!("hlqnsbe", no1());
  }

  #[test]
  fn no2_test() {
    assert_eq!(1993, no2());
  }
}

pub fn no1() -> String {
  let input = include_str!("../inputs/input07").trim_right();
  run1(input)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input07").trim_right();
  run2(input)
}

fn run1(list: &str) -> String {
  let re = Regex::new(r"^(.+) \([[:digit:]]+\)( -> )?(.*)$").unwrap();

  let mut all_programs = HashSet::new();
  let mut top_programs = HashSet::new();

  for line in list.lines() {
    let cap = re.captures(line).unwrap();
    all_programs.insert(cap[1].to_string());

    for prog in cap[3].split(", ") {
      top_programs.insert(prog.to_string());
    }
  }

  all_programs.sub(&top_programs).into_iter().nth(0).unwrap()
}

fn run2(list: &str) -> i32 {
  let mut current = run1(list);
  let mut result = 0;

  loop {
    let mut hash = HashMap::new();
    let children = get_children(list, current.clone());
    for c in children.clone() {
      hash.insert(c.clone(), get_sum(list, c));
    }

    let majority = get_majority(&hash);

    let mut completed = true;
    for c in children {
      let val = *hash.get(&c).unwrap();
      if val != majority {
        completed = false;
        result = val - majority;
        current = c;
        break;
      }
    }

    if completed {
      return get_number(list, current) - result;
    }
  }
}

fn get_children(list: &str, program: String) -> Vec<String> {
  lazy_static! {
    static ref RE: Regex =
      Regex::new(r"^(.+) \(([[:digit:]]+)\)( -> )?(.*)$").unwrap();
  }

  for line in list.lines() {
    let cap = RE.captures(line).unwrap();
    let name = cap[1].to_string();
    if name == program {
      return cap[4].split(", ").map(String::from).collect();
    }
  }

  panic!("Program not inside");
}

fn get_sum(list: &str, program: String) -> i32 {
  lazy_static! {
    static ref RE: Regex =
      Regex::new(r"^(.+) \(([[:digit:]]+)\)( -> )?(.*)$").unwrap();
  }

  for line in list.lines() {
    let cap = RE.captures(line).unwrap();
    let name = cap[1].to_string();
    if name == program {
      let mut sum = cap[2].parse().unwrap();

      for s in cap[4].split(", ") {
        if !s.is_empty() {
          sum += get_sum(list, s.to_string());
        }
      }

      return sum;
    }
  }

  panic!("Program not inside");
}

fn get_number(list: &str, program: String) -> i32 {
  lazy_static! {
    static ref RE: Regex =
      Regex::new(r"^(.+) \(([[:digit:]]+)\)( -> )?(.*)$").unwrap();
  }

  for line in list.lines() {
    let cap = RE.captures(line).unwrap();
    let name = cap[1].to_string();
    if name == program {
      return cap[2].parse().unwrap();
    }
  }

  panic!("Program not inside");
}

fn get_majority(h: &HashMap<String, i32>) -> i32 {
  let mut k = h.keys();
  let first = k.next().unwrap();
  let second = k.next().unwrap();
  if h.get(first) == h.get(second) {
    return *h.get(first).unwrap();
  }

  let third = k.next().unwrap();

  if h.get(second) == h.get(third) {
    return *h.get(second).unwrap();
  } else if h.get(third) == h.get(first) {
    return *h.get(third).unwrap();
  } else {
    panic!("GG");
  }
}
