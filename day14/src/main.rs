use std::collections::HashSet;

const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("./input.txt");

fn main() {
  let sample = parse(SAMPLE);
  let input = parse(INPUT);

  println!("sample 1: {}", falls_down(&sample, None));
  println!("input 1: {}", falls_down(&input, None));

  println!("sample 2: {}", falls_down(&sample, Some(2)));
  println!("input 2: {}", falls_down(&input, Some(2)));
}

#[derive(Debug)]
struct CompositeLine {
  lines: Vec<LineStrip>,
}

impl CompositeLine {
  fn collides(&self, p: (i32, i32)) -> bool {
    self.lines.iter().any(|line| line.collides(p))
  }

  fn max_y(&self) -> i32 {
    self.lines.iter().map(LineStrip::max_y).max().unwrap()
  }
}

#[derive(Debug)]
struct LineStrip {
  points: Vec<(i32, i32)>,
}

impl LineStrip {
  fn collides(&self, (px, py): (i32, i32)) -> bool {
    self
      .points
      .iter()
      .zip(self.points.iter().skip(1))
      .any(|((ax, ay), (bx, by))| {
        let rx = if ax <= bx { *ax..=*bx } else { *bx..=*ax };
        let ry = if ay <= by { *ay..=*by } else { *by..=*ay };
        rx.contains(&px) && ry.contains(&py)
      })
  }

  fn max_y(&self) -> i32 {
    self.points.iter().map(|(_, y)| *y).max().unwrap()
  }
}

fn parse(input: &str) -> CompositeLine {
  let lines = input
    .lines()
    .map(|line| {
      let points = line
        .split("->")
        .map(|p| {
          let mut iter = p.trim().split(',');
          (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
          )
        })
        .collect();
      LineStrip { points }
    })
    .collect();

  CompositeLine { lines }
}

fn falls_down(rocks: &CompositeLine, y_bias: Option<i32>) -> u32 {
  let mut sand: (i32, i32) = (500, 0);
  let mut rest: HashSet<(i32, i32)> = HashSet::new();
  let max_y = rocks.max_y();

  let collide_floor = |next_sand: &(i32, i32)| {
    if let Some(y_bias) = y_bias {
      next_sand.1 == max_y + y_bias
    } else {
      false
    }
  };

  loop {
    if y_bias.is_none() && sand.1 >= max_y {
      break;
    }

    let next_sand = (sand.0, sand.1 + 1); // down
    if !(rocks.collides(next_sand) || rest.contains(&next_sand) || collide_floor(&next_sand)) {
      sand = next_sand;
      continue;
    }

    let next_sand = (sand.0 - 1, sand.1 + 1); // left down
    if !(rocks.collides(next_sand) || rest.contains(&next_sand) || collide_floor(&next_sand)) {
      sand = next_sand;
      continue;
    }

    let next_sand = (sand.0 + 1, sand.1 + 1); // right down
    if !(rocks.collides(next_sand) || rest.contains(&next_sand) || collide_floor(&next_sand)) {
      sand = next_sand;
      continue;
    }

    rest.insert(sand);

    if sand == (500, 0) {
      break;
    }

    sand = (500, 0);
  }

  rest.len() as _
}
