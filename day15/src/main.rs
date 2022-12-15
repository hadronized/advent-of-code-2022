use std::{collections::HashSet, ops::RangeInclusive};

const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("./input.txt");

fn main() {
  let sample = parse(SAMPLE);
  let input = parse(INPUT);

  println!("sample 1: {}", coverage_for_y(&sample, 10));
  println!("input 1: {}", coverage_for_y(&input, 2000000));

  println!("sample 2: {:?}", edge_testing(&sample, 20));
  println!("input 2: {:?}", edge_testing(&input, 4000000));
}

fn parse(input: &str) -> Vec<[(i64, i64); 2]> {
  fn get_pair<'a>(mut iter: impl Iterator<Item = &'a str>) -> (i64, i64) {
    let x = iter.next().unwrap().parse().unwrap();
    let y = iter.next().unwrap().trim()["y=".len()..].parse().unwrap();

    (x, y)
  }

  input
    .lines()
    .map(|line| {
      let mut parts = line.split(':');

      let left = parts.next().unwrap()["Sensor at x=".len()..].split(',');
      let (sx, sy) = get_pair(left);

      let right = parts.next().unwrap().trim()["closest beacon is at x=".len()..].split(',');
      let (bx, by) = get_pair(right);

      [(sx, sy), (bx, by)]
    })
    .collect()
}

fn manhattan(&(ax, ay): &(i64, i64), &(bx, by): &(i64, i64)) -> i64 {
  (bx - ax).abs() + (by - ay).abs()
}

fn merge_range(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> Option<RangeInclusive<i64>> {
  if !(a.contains(b.start()) || b.contains(a.start())) {
    return None;
  }

  let start = *a.start().min(b.start());
  let end = *a.end().max(b.end());

  Some(start..=end)
}

fn count_no_beacon(
  forbidden: &HashSet<(i64, i64)>,
  range: &RangeInclusive<i64>,
  y_filter: i64,
) -> i64 {
  *range.end() - *range.start() + 1
    - forbidden
      .iter()
      .filter(|(x, y)| *y == y_filter && range.contains(x))
      .count() as i64
}

fn coverage_for_y(data: &Vec<[(i64, i64); 2]>, y_filter: i64) -> i64 {
  let forbidden = data.iter().flatten().copied().collect::<HashSet<_>>();
  let mut ranges: Vec<_> = Vec::new();

  for [sensor, beacon] in data {
    let r = manhattan(sensor, beacon);
    let range_y = sensor.1 - r..=sensor.1 + r;

    if range_y.contains(&y_filter) {
      let dy = (y_filter - sensor.1).abs();
      let r2 = r - dy;
      let range_x = sensor.0 - r2..=sensor.0 + r2;
      ranges.push(range_x);
    }
  }

  ranges.sort_by_key(|range| *range.start());

  let mut new_ranges = Vec::new();
  let mut current = ranges[0].clone();

  for range in &ranges[1..] {
    if let Some(merged) = merge_range(&current, range) {
      current = merged;
    } else {
      new_ranges.push(current);
      current = range.clone();
    }
  }

  ranges = new_ranges;
  ranges.push(current);

  ranges
    .iter()
    .map(|r| count_no_beacon(&forbidden, r, y_filter))
    .sum()
}

#[derive(Debug, Clone)]
struct EdgeIter {
  rx: i64,
  ry: i64,
  current: (i64, i64),
  end: (i64, i64),
}

impl Iterator for EdgeIter {
  type Item = (i64, i64);

  fn next(&mut self) -> Option<Self::Item> {
    if self.current >= self.end {
      return None;
    }

    let current = self.current;
    self.current.0 += self.rx;
    self.current.1 += self.ry;

    Some(current)
  }
}

fn iter_edge(a: (i64, i64), b: (i64, i64)) -> EdgeIter {
  let rx = if a.0 <= b.0 { 1 } else { -1 };
  let ry = if a.1 <= b.1 { 1 } else { -1 };

  EdgeIter {
    rx,
    ry,
    current: a,
    end: b,
  }
}

fn sensor_edges(sensor: &(i64, i64), r: i64) -> [EdgeIter; 4] {
  let a = (sensor.0 - r, sensor.1);
  let b = (sensor.0, sensor.1 - r);
  let c = (sensor.0 + r, sensor.1);
  let d = (sensor.0, sensor.1 + r);

  [
    iter_edge(a, b),
    iter_edge(b, c),
    iter_edge(c, d),
    iter_edge(d, a),
  ]
}

// better algorithm: edge walking
fn edge_testing(data: &Vec<[(i64, i64); 2]>, xy_filter: i64) -> i64 {
  let sensors: Vec<_> = data
    .iter()
    .map(|[sensor, beacon]| {
      let r = manhattan(sensor, beacon);
      (sensor, r)
    })
    .collect();

  let edges_per_sensor: Vec<_> = data
    .iter()
    .map(|[sensor, beacon]| {
      let r = manhattan(sensor, beacon) + 1;
      sensor_edges(sensor, r)
    })
    .collect();

  for edges in edges_per_sensor {
    for edge in edges {
      for p in edge {
        if p.0 >= 0 && p.0 <= xy_filter && p.1 >= 0 && p.1 <= xy_filter {
          if sensors
            .iter()
            .all(|(sensor, r)| manhattan(&p, *sensor) > *r)
          {
            return p.0 * 4000000 + p.1;
          }
        }
      }
    }
  }

  0
}
