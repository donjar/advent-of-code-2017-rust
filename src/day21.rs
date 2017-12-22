use regex::Regex;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no1_sample_test() {
    let input = indoc!(
      "../.# => ##./#../...
       .#./..#/### => #..#/..../..../#..#"
    );
    assert_eq!(12, run1(input, 2));
  }

  #[ignore]
  #[test]
  fn no2_sample_test() {
    let input = indoc!(
      "../.# => ##./#../...
       .#./..#/### => #..#/..../..../#..#"
    );
    assert_eq!(12, run2(input, 2));
  }

  #[test]
  fn no1_test() {
    assert_eq!(120, no1());
  }

  #[ignore]
  #[test]
  fn no2_test() {
    assert_eq!(3833504, no2());
  }
}

pub fn no1() -> i32 {
  let input = include_str!("../inputs/input21").trim_right();
  run1(input, 5)
}

pub fn no2() -> i32 {
  let input = include_str!("../inputs/input21").trim_right();
  run2(input, 5)
}

fn run1(rules: &str, iterations: i32) -> i32 {
  let mut art = Art::new(rules);
  for i in 0..iterations {
    art.next();
  }
  art.count_ons()
}

fn run2(rules: &str, iterations: i32) -> i32 {
  let re = Regex::new(r"^(.*) => (.*)$").unwrap();
  0
}

struct Art {
  rules: HashMap<String, String>,
  picture: Vec<Vec<bool>>,
}

#[derive(Clone)]
struct Component {
  component: Vec<Vec<bool>>,
}

impl Art {
  fn new(rule_list: &str) -> Art {
    let re = Regex::new(r"^(.*) => (.*)$").unwrap();
    let mut rules_map = HashMap::new();

    for rule in rule_list.lines() {
      for cap in re.captures_iter(rule) {
        rules_map.insert(String::from(&cap[1]), String::from(&cap[2]));
      }
    }

    let picture = vec![
      vec![false, true, false],
      vec![false, false, true],
      vec![true, true, true],
    ];

    Art {
      rules: rules_map,
      picture,
    }
  }

  fn next(&mut self) {
    self.picture = Art::combine_components(
      self
        .components()
        .iter()
        .map(|ref r| {
          r.iter()
            .map(|ref c| self.transform(c.clone().clone()))
            .collect()
        })
        .collect(),
    );
  }

  fn count_ons(&self) -> i32 {
    self
      .picture
      .iter()
      .map(|ref v| {
        v.iter().map(|&b| if b { 1 } else { 0 }).sum::<i32>()
      })
      .sum()
  }

  fn components(&self) -> Vec<Vec<Component>> {
    let length = self.picture.len();
    if length % 2 == 0 {
      self.split_into_components(2)
    } else if length % 3 == 0 {
      self.split_into_components(3)
    } else {
      panic!("Length should be divisible by 2 or 3");
    }
  }

  fn split_into_components(
    &self,
    component_size: usize,
  ) -> Vec<Vec<Component>> {
    let mut result = Vec::new();
    let height = self.picture.len();
    let width = self.picture[0].len();

    for i in 0..(height / component_size) {
      let mut row = Vec::new();
      for j in 0..(width / component_size) {
        let component = Component::new(
          &self.picture,
          i * component_size,
          (i + 1) * component_size,
          j * component_size,
          (j + 1) * component_size,
        );
        row.push(component);
      }
      result.push(row);
    }

    result
  }

  fn combine_components(components: Vec<Vec<Component>>) -> Vec<Vec<bool>> {
    let mut result = Vec::new();

    for i in 0..components.len() {
      let row_len = components[i][0].component.len();

      for j in 0..row_len {
        let row = components[i]
          .iter()
          .flat_map(|ref c| c.component[j].clone())
          .collect();
        result.push(row);
      }
    }

    result
  }

  fn transform(&self, component: Component) -> Component {
    // Given a component, split into 8 possibilities
    for c in component.symmetry() {
      // Convert the component into dot-and-hash
      let c_text = c.text_form();
      // Match with rulebook
      if let Some(result) = self.rules.get(&c_text) {
        // Transform component
        return Component::from(result.clone());
      }
    }

    panic!("Transformation not found");
  }
}

impl Component {
  fn new(
    matrix: &Vec<Vec<bool>>,
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
  ) -> Component {
    let mut component = Vec::new();
    for i in xmin..xmax {
      let mut row = Vec::new();
      for j in ymin..ymax {
        row.push(matrix[i][j]);
      }
      component.push(row);
    }

    Component { component }
  }

  fn symmetry(&self) -> Vec<Component> {
    vec![
      self.clone(),
      self.right_turn().clone(),
      self.full_turn().clone(),
      self.left_turn().clone(),
      self.hor_flip().clone(),
      self.right_turn().hor_flip().clone(),
      self.full_turn().hor_flip().clone(),
      self.left_turn().hor_flip().clone(),
    ]
  }

  fn text_form(&self) -> String {
    self
      .component
      .iter()
      .map(|c| {
        c.iter()
          .map(|&r| if r { "#" } else { "." })
          .collect::<Vec<&str>>()
          .concat()
      })
      .collect::<Vec<String>>()
      .join("/")
  }

  fn from(str_form: String) -> Component {
    let component = str_form
      .split("/")
      .map(|seg| seg.chars().map(|c| c == '#').collect())
      .collect();
    Component { component }
  }

  fn right_turn(&self) -> Component {
    let mut component = Vec::new();
    for i in 0..self.component.len() {
      component.push(self.component.iter().rev().map(|r| r[i]).collect());
    }
    Component { component }
  }

  fn full_turn(&self) -> Component {
    self.right_turn().right_turn()
  }

  fn left_turn(&self) -> Component {
    self.right_turn().right_turn().right_turn()
  }

  fn hor_flip(&self) -> Component {
    let component = self
      .component
      .iter()
      .map(|r| r.iter().rev().cloned().collect())
      .collect();
    Component { component }
  }
}
