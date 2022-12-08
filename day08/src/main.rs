const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("./input.txt");

fn main() {
  let sample = parse(SAMPLE);
  let input = parse(INPUT);

  println!("sample 1: {}", solve1(&sample));
  println!("input 1: {}", solve1(&input));
}

fn parse(input: &str) -> Vec<Vec<i16>> {
  input
    .lines()
    .map(|line| line.bytes().map(|x| x as _).collect())
    .collect()
}

fn solve1(data: &Vec<Vec<i16>>) -> u32 {
  let mut status: Vec<Vec<bool>> = data
    .iter()
    .map(|trees| trees.iter().map(|_| false).collect())
    .collect();
  let width = data[0].len();
  let height = data.len();
  let mut visible = 0;

  // horizontal
  for y in 0..height {
    // left to right
    let mut prev_height: i16 = -1;

    for x in 0..width {
      if data[y][x] as i16 <= prev_height {
        // tree not visible from this angle, stop
        continue;
      }

      if !status[y][x] {
        status[y][x] = true;
        visible += 1;
      }

      prev_height = data[y][x];
    }

    // right to left
    prev_height = -1;
    for x in 0..width {
      let x = width - 1 - x;

      if data[y][x] as i16 <= prev_height {
        // tree not visible from this angle, stop
        continue;
      }

      if !status[y][x] {
        status[y][x] = true;
        visible += 1;
      }

      prev_height = data[y][x];
    }
  }

  for x in 0..width {
    let mut prev_height: i16 = -1;

    // top to bottom
    for y in 0..height {
      if data[y][x] as i16 <= prev_height {
        // tree not visible from this angle, stop
        continue;
      }

      if !status[y][x] {
        status[y][x] = true;
        visible += 1;
      }

      prev_height = data[y][x];
    }

    let mut prev_height: i16 = -1;

    // bottom to top
    for y in 0..height {
      let y = height - 1 - y;
      if data[y][x] as i16 <= prev_height {
        // tree not visible from this angle, stop
        continue;
      }

      if !status[y][x] {
        status[y][x] = true;
        visible += 1;
      }

      prev_height = data[y][x];
    }
  }

  visible
}
