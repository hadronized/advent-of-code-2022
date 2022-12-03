use itertools::Itertools;
use std::collections::HashMap;

const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("./input.txt");

fn main() {
  let sample = parse(SAMPLE);
  let input = parse(INPUT);

  println!("sample 1: {}", solve1(&sample));
  println!("part 1: {}", solve1(&input));

  println!("sample 2: {}", solve2(&sample));
  println!("part 2: {}", solve2(&input));
}

fn parse(input: &str) -> Vec<Vec<u8>> {
  input.lines().map(|line| line.bytes().collect()).collect()
}

fn priority(item: u8) -> u32 {
  if item < b'a' {
    (item - b'A' + 27) as _
  } else {
    (item - b'a' + 1) as _
  }
}

fn solve1(data: &Vec<Vec<u8>>) -> u32 {
  let mut sum = 0;

  for line in data {
    let (left, right) = line.split_at(line.len() / 2);

    let mut right_counts: HashMap<u8, usize> = HashMap::new();
    for item in right {
      *right_counts.entry(*item).or_default() += 1;
    }

    for item in left {
      if right_counts.contains_key(item) {
        sum += priority(*item);
        break;
      }
    }
  }

  sum
}

fn solve2(data: &Vec<Vec<u8>>) -> u32 {
  let mut sum = 0;
  for group in data.chunks(3) {
    let mut seen_in_group: HashMap<u8, usize> = HashMap::new();

    for line in group {
      for item in line.iter().unique() {
        *seen_in_group.entry(*item).or_default() += 1;
      }
    }

    let item = seen_in_group
      .into_iter()
      .filter(|(_, count)| *count == 3)
      .map(|(item, _)| item)
      .next()
      .unwrap();
    sum += priority(item);
  }

  sum
}
