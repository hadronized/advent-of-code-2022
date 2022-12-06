use std::collections::HashMap;

const INPUT: &'static str = include_str!("./input.txt");

fn main() {
  println!("input 1: {}", solve(4, INPUT));
  println!("input 2: {}", solve(14, INPUT));
}

fn solve(len: usize, input: &str) -> u32 {
  let bytes = input.as_bytes();
  let mut seen: HashMap<u8, usize> = HashMap::new();

  let mut i = 0;
  while i < bytes.len() - len {
    for j in 0..len {
      let c = bytes[i + j];

      if let Some(k) = seen.get(&c) {
        i = k + 1;
        break;
      }

      seen.insert(c, i + j);
    }

    if seen.len() == len {
      return (i + len) as u32;
    }

    seen.clear();
  }

  0
}
