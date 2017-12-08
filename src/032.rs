use std::collections::HashMap;
fn main() {
  let input = 325489;
  println!("{}", run(input));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate(i32, i32);

impl Coordinate {
  fn next(&self) -> Coordinate {
    if self.1 <= 0 && self.1 <= self.0 && self.0 <= -1 * self.1 {
      // (-2, -2), (-1, -2), (0, -2), (1, -2), (2, -2): go right
      Coordinate(self.0 + 1, self.1)
    } else if self.0 > 0 && -1 * self.0 < self.1 && self.1 < self.0 {
      // (2, -1), (2, 0), (2, 1): go up
      Coordinate(self.0, self.1 + 1)
    } else if self.1 > 0 && -1 * self.1 < self.0 && self.0 <= self.1 {
      // (2, 2), (1, 2), (0, 2), (-1, 2): go left
      Coordinate(self.0 - 1, self.1)
    } else {
      // go down
      Coordinate(self.0, self.1 - 1)
    }
  }

  fn neighbors(&self) -> [Coordinate; 8] {
    [
      Coordinate(self.0 + 1, self.1 + 1),
      Coordinate(self.0, self.1 + 1),
      Coordinate(self.0 - 1, self.1 + 1),
      Coordinate(self.0 + 1, self.1),
      Coordinate(self.0 - 1, self.1),
      Coordinate(self.0 + 1, self.1 - 1),
      Coordinate(self.0, self.1 - 1),
      Coordinate(self.0 - 1, self.1 - 1),
    ]
  }
}

fn run(input: i32) -> i32 {
  let mut coordinate_map = HashMap::new();

  let mut current_coordinate = Coordinate(0, 0);
  let mut current_value = 1;

  while current_value < input {
    coordinate_map.insert(current_coordinate, current_value);

    current_coordinate = current_coordinate.next();
    current_value = current_coordinate
      .neighbors()
      .iter()
      .map(|c| coordinate_map.get(c).unwrap_or(&0))
      .sum()
  }

  current_value
}
