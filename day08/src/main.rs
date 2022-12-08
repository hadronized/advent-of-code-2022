const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("./input.txt");

fn main() {
  let sample = parse(SAMPLE);
  let input = parse(INPUT);

  println!("sample 1: {}", solve1(&sample));
  println!("input 1: {}", solve1(&input));

  println!("sample 2: {}", solve2(&sample));
  println!("input 2: {}", solve2(&input));
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

  // vertical
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

fn solve2(data: &Vec<Vec<i16>>) -> u32 {
  let width = data[0].len();
  let height = data.len();
  let mut scores: Vec<Vec<u32>> = data
    .iter()
    .map(|trees| trees.iter().map(|_| 1).collect())
    .collect();

  for y in 0..height {
    for x in 0..width {
      let tree_height = data[y][x];
      let mut trees = 0;

      // look left
      for k in 1.. {
        if (x as isize - k as isize) < 0 {
          break;
        }

        trees += 1;

        if data[y][x - k] >= tree_height {
          break;
        }
      }

      scores[y][x] *= trees;
      trees = 0;

      // look right
      for k in 1.. {
        if (x as isize + k as isize) > width as isize - 1 {
          break;
        }

        trees += 1;

        if data[y][x + k] >= tree_height {
          break;
        }
      }

      scores[y][x] *= trees;
      trees = 0;

      // look up
      for k in 1.. {
        if (y as isize - k as isize) < 0 {
          break;
        }

        trees += 1;

        if data[y - k][x] >= tree_height {
          break;
        }
      }

      scores[y][x] *= trees;
      trees = 0;

      // look down
      for k in 1.. {
        if (y as isize + k as isize) > height as isize - 1 {
          break;
        }

        trees += 1;

        if data[y + k][x] >= tree_height {
          break;
        }
      }

      scores[y][x] *= trees;
    }
  }

  scores.iter().flatten().copied().max().unwrap()
}
