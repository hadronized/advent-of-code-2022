use std::ops::RangeInclusive;

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

fn parse(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
  input
    .lines()
    .map(|line| {
      let mut ranges = line.split(",").map(|range| {
        let mut digits = range.split('-').flat_map(|digits| digits.parse());
        digits.next().unwrap()..=digits.next().unwrap()
      });

      (ranges.next().unwrap(), ranges.next().unwrap())
    })
    .collect()
}

fn fully_contains(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
  a.start() <= b.start() && b.end() <= a.end()
}

fn overlaps(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
  a.end() >= b.start() && a.start() <= b.start()
}

fn solve1(data: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> u32 {
  data
    .iter()
    .filter(|(a, b)| fully_contains(a, b) || fully_contains(b, a))
    .count() as _
}

fn solve2(data: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> u32 {
  data
    .iter()
    .filter(|(a, b)| overlaps(a, b) || overlaps(b, a))
    .count() as _
}
