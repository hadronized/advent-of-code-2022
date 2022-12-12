use std::{
  cmp::Ordering,
  collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
};

const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("./input.txt");

fn main() {
  let sample = parse(SAMPLE);
  let input = parse(INPUT);

  let (sample_start, sample_end) = find_start_end(&sample);
  let (input_start, input_end) = find_start_end(&input);

  println!("sample 1: {}", find_path(&sample, sample_start, sample_end));
  println!("input 1: {}", find_path(&input, input_start, input_end));

  println!("sample 2: {}", solve2(&sample));
  println!("input 2: {}", solve2(&input));
}

fn parse(input: &str) -> Vec<Vec<u8>> {
  input.lines().map(|line| line.bytes().collect()).collect()
}

fn find_start_end(heightmap: &Vec<Vec<u8>>) -> ((usize, usize), (usize, usize)) {
  let mut start = (0, 0);
  let mut end = (0, 0);

  for y in 0..heightmap.len() {
    for x in 0..heightmap[y].len() {
      match heightmap[y][x] {
        b'S' => start = (x, y),
        b'E' => end = (x, y),
        _ => (),
      }

      if start != (0, 0) && end != (0, 0) {
        return (start, end);
      }
    }
  }

  (start, end)
}

#[derive(Clone, Copy, Debug, Hash)]
pub struct Vertex {
  pos: (usize, usize),
  weight: u32,
}

impl Vertex {
  pub fn new(pos: (usize, usize), weight: u32) -> Self {
    Self { pos, weight }
  }
}

impl PartialEq for Vertex {
  fn eq(&self, other: &Self) -> bool {
    self.weight.eq(&other.weight)
  }
}

impl Eq for Vertex {}

impl PartialOrd for Vertex {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.weight.partial_cmp(&self.weight)
  }
}

impl Ord for Vertex {
  fn cmp(&self, other: &Self) -> Ordering {
    other.weight.cmp(&self.weight)
  }
}

fn elev(x: u8) -> u8 {
  match x {
    b'S' => 0,
    b'E' => 25,
    _ => x - b'a',
  }
}

fn find_path(heightmap: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> usize {
  let mut seen = HashSet::new();
  let mut parents = HashMap::new();
  parents.insert(start, (start, 0));

  let mut next = BinaryHeap::new();
  next.push(Vertex::new(start, 0));

  // walk the heightmap
  while let Some(vertex) = next.pop() {
    if seen.contains(&vertex.pos) {
      continue;
    }

    seen.insert(vertex.pos);

    let (x, y) = vertex.pos;
    let elevation = elev(heightmap[y][x]);
    let (_, dist) = parents.get(&vertex.pos).unwrap().clone();

    if vertex.pos == end {
      break;
    }

    for neighbor @ (nx, ny) in neighbors(heightmap, vertex.pos) {
      if elev(heightmap[ny][nx]) > elevation + 1 {
        // we cannot visit that thing yet, continue
        continue;
      }

      // update the distance to this neighbor if we have a faster route
      match parents.entry(neighbor) {
        Entry::Occupied(mut entry) => {
          if entry.get().1 > dist + 1 {
            entry.insert((vertex.pos, dist + 1));
          }
        }

        Entry::Vacant(entry) => {
          entry.insert((vertex.pos, dist + 1));
        }
      }

      next.push(Vertex::new(neighbor, dist));
    }
  }

  // build back the path
  let mut current = end;
  let mut len = 0;
  while current != start {
    if let Some(parent) = parents.remove(&current) {
      len += 1;
      current = parent.0;
    } else {
      return usize::MAX;
    }
  }

  len
}

fn neighbors(heightmap: &Vec<Vec<u8>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
  let mut out = Vec::new();

  if x > 0 {
    out.push((x - 1, y));
  }

  if x < heightmap[0].len() - 1 {
    out.push((x + 1, y));
  }

  if y > 0 {
    out.push((x, y - 1));
  }

  if y < heightmap.len() - 1 {
    out.push((x, y + 1));
  }

  out
}

fn solve2(heightmap: &Vec<Vec<u8>>) -> usize {
  let (_, end) = find_start_end(heightmap);

  let c = heightmap
    .iter()
    .enumerate()
    .flat_map(move |(y, line)| {
      line.iter().enumerate().filter_map(
        move |(x, &h)| {
          if elev(h) == 0 {
            Some((x, y))
          } else {
            None
          }
        },
      )
    })
    .collect::<Vec<_>>();

  c.into_iter()
    .map(|start| find_path(heightmap, start, end))
    .min()
    .unwrap()
}
