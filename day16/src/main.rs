//! This shit doesn’t produce the same output as the example, I get more pressure, so there must be a bug somewhere or I
//! have a super quantic algorithm LOL.
use std::{
  collections::{BTreeMap, HashMap, HashSet},
  iter::once,
};

const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("./input.txt");

fn main() {
  let sample = parse(SAMPLE);
  let input = parse(INPUT);

  println!("sample part 1: {}", solve1(&sample));
  println!("input part 1: {}", solve1(&input));
}

#[derive(Debug)]
struct Valve {
  rate: i32,
  reachable: HashSet<String>,
}

fn parse(input: &str) -> HashMap<String, Valve> {
  input
    .lines()
    .map(|line| {
      let name = line["Valve ".len().."Valve ".len() + 2].to_owned();

      let mut parts = line["Valve XX has flow rate=".len()..].split(';');
      let rate = parts.next().unwrap().parse().unwrap();

      let parts = parts.next().unwrap().split("valve").skip(1).next().unwrap();

      let reachable: _ = if parts.as_bytes()[0] == b's' {
        parts[2..]
          .split(',')
          .map(|name| name.trim().to_owned())
          .collect()
      } else {
        once(parts[1..3].to_owned()).collect()
      };

      (name, Valve { rate, reachable })
    })
    .collect()
}

fn current_pressure(opened: &HashMap<String, i32>) -> i32 {
  opened.values().sum()
}

type Distance = u32;

#[derive(Debug)]
struct Paths {
  parents: HashMap<String, (String, Distance)>,
}

impl Paths {
  fn new(valves: &HashMap<String, Valve>) -> Self {
    // TODO: not very optimal
    for current in valves.keys() {}

    Self { parents }
  }
}

fn solve1(data: &HashMap<String, Valve>) -> i32 {
  todo!()
}
