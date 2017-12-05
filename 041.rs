use std::collections::HashSet;

fn main() {
  let input = include_str!("input4");
  let trimmed_input = &input[..input.len() - 1]; // remove last newline

  println!("{}", run(trimmed_input));
}

fn run(passphrases: &str) -> u32 {
  let lines = passphrases.lines();

  lines
    .map(|passphrase| {
      let mut words = HashSet::new();

      for w in passphrase.split(" ") {
        if words.contains(w) {
          return 0;
        }
        words.insert(w);
      }

      return 1;
    })
    .sum()
}
