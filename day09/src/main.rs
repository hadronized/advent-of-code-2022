use std::{collections::HashSet, iter::repeat};

const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("input.txt");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Dir {
  Left,
  Right,
  Up,
  Down,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Move {
  dir: Dir,
  unit: usize,
}

fn main() {
  let sample = parse(SAMPLE);
  let input = parse(INPUT);

  println!("sample 1: {}", solve(&sample, 2));
  println!("input 1: {}", solve(&input, 2));

  println!("sample 2: {}", solve(&sample, 10));
  println!("input 2: {}", solve(&input, 10));
}

fn parse<'a>(input: &'a str) -> Vec<Move> {
  input
    .lines()
    .map(|line| {
      let mut split = line.split_whitespace();
      let dir = match split.next().unwrap() {
        "L" => Dir::Left,
        "R" => Dir::Right,
        "U" => Dir::Up,
        "D" => Dir::Down,
        _ => unreachable!(),
      };

      let unit = split.next().and_then(|x| x.parse().ok()).unwrap();

      Move { dir, unit }
    })
    .collect()
}

fn whip(m: Dir, knots: &mut [(i32, i32)]) {
  match m {
    Dir::Left => {
      knots[0].0 -= 1;
    }

    Dir::Right => {
      knots[0].0 += 1;
    }

    Dir::Up => {
      knots[0].1 -= 1;
    }

    Dir::Down => {
      knots[0].1 += 1;
    }
  }

  for i in 1..knots.len() {
    let head = knots[i - 1];
    let tail = &mut knots[i];

    if head.0 != tail.0
      && head.1 != tail.1
      && ((head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1)
    {
      tail.0 += (head.0 - tail.0).signum();
      tail.1 += (head.1 - tail.1).signum();
    } else if tail.0 == head.0 && (head.1 - tail.1).abs() > 1 {
      tail.1 += (head.1 - tail.1).signum();
    } else if (head.0 - tail.0).abs() > 1 {
      tail.0 += (head.0 - tail.0).signum();
    }
  }
}

fn solve(data: &Vec<Move>, count: usize) -> u32 {
  let mut seen = HashSet::new();
  let mut knots = vec![(0, 0); count];

  for m in data.iter().flat_map(|m| repeat(m.dir).take(m.unit)) {
    whip(m, &mut knots);
    seen.insert(knots[knots.len() - 1]);
  }

  seen.len() as _
}
