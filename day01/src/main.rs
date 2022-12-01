use std::cmp::Reverse;

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

fn parse(input: &str) -> Vec<Vec<u32>> {
  input
    .split("\n\n")
    .map(|elf| elf.split('\n').flat_map(str::parse).collect::<Vec<_>>())
    .collect()
}

fn solve1(data: &Vec<Vec<u32>>) -> u32 {
  data
    .iter()
    .map(|calories| calories.iter().sum())
    .max()
    .unwrap_or(0)
}

fn solve2(data: &Vec<Vec<u32>>) -> u32 {
  let mut calories = data
    .iter()
    .map(|calories| calories.iter().sum())
    .collect::<Vec<_>>();

  calories.sort_by_key(|&c| Reverse(c));

  calories.iter().take(3).sum()
}
