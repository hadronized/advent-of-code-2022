const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("./input.txt");

fn main() {
  let sample = parse(SAMPLE);
  let input = parse(INPUT);

  println!("sample 1 {}", solve1(&sample));
  println!("part 1 {}", solve1(&input));

  println!("sample 2 {}", solve2(&sample));
  println!("part 2 {}", solve2(&input));
}

fn parse(input: &str) -> Vec<(u8, u8)> {
  input
    .lines()
    .map(|line| {
      let bytes = line.as_bytes();
      (bytes[0], bytes[2])
    })
    .collect()
}

fn compute_score(a: u8, b: u8) -> u32 {
  let score = (b - b'X') as u32 + 1;
  if a - b'A' == b - b'X' {
    score + 3
  } else if a - b'A' == (b - b'X' + 1) % 3 {
    score
  } else {
    score + 6
  }
}

fn decide(a: u8, end: u8) -> u8 {
  match end {
    b'X' => ((a - b'A').checked_sub(1).unwrap_or(2)) % 3 + b'X',
    b'Y' => a - b'A' + b'X',
    b'Z' => (a - b'A' + 1) % 3 + b'X',
    _ => unreachable!(),
  }
}

fn solve1(data: &Vec<(u8, u8)>) -> u32 {
  data
    .iter()
    .fold(0, |score, (a, b)| score + compute_score(*a, *b))
}

fn solve2(data: &Vec<(u8, u8)>) -> u32 {
  data.iter().fold(0, |score, (a, end)| {
    score + compute_score(*a, decide(*a, *end))
  })
}
