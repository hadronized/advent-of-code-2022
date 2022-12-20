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

fn next_to_open(
  data: &HashMap<String, Valve>,
  current: &str,
  p1: &Paths,
  opened: &HashMap<String, i32>,
  minute: i32,
) -> Option<(i32, String)> {
  let mut next = None;

  for (dist1, neighbor1) in p1
    .reachable()
    .filter(|(_, name)| !opened.contains_key(*name) && *name != current)
  {
    let rate1 = data.get(neighbor1).unwrap().rate;

    if rate1 == 0 {
      continue;
    }

    let potential1 = (30 - minute - dist1 - 1) * rate1 as i32;

    let p2 = Paths::new(data, neighbor1);
    for (dist2, neighbor2) in p2
      .reachable()
      .filter(|(_, name)| !opened.contains_key(*name) && ![current].contains(name))
    {
      let rate2 = data.get(neighbor2).unwrap().rate;

      if rate2 == 0 {
        continue;
      }

      let potential2 = (30 - minute - dist1 - dist2 - 2) * rate2 as i32;
      let total_potential = potential1 + potential2;

      let (p, n) = next.get_or_insert_with(|| (total_potential, neighbor1.to_owned()));
      if total_potential > *p {
        *p = total_potential;
        *n = neighbor1.to_owned();
      }
    }
  }

  next
}

#[derive(Debug)]
struct Paths {
  start: String,
  parents: HashMap<String, (i32, String)>,
}

impl Paths {
  fn new(data: &HashMap<String, Valve>, start: &str) -> Self {
    let mut seen = HashSet::new();
    let mut parents = HashMap::<String, (i32, String)>::new();
    let mut next = BTreeMap::new();

    parents.insert(start.to_owned(), (0, start.to_owned()));
    next.insert(start, 0);

    while let Some((name, _weight)) = next.pop_first() {
      if seen.contains(name) {
        continue;
      }

      seen.insert(name);

      let weight = parents.get(name).unwrap().0;

      for neighbor in data.get(name).unwrap().reachable.iter() {
        let weight = weight + 1;
        next.insert(neighbor, weight);
        let p = parents
          .entry(neighbor.to_owned())
          .or_insert((i32::MAX, String::new()));

        if weight < p.0 {
          p.0 = weight;
          p.1 = name.to_owned();
        }
      }
    }

    Paths {
      start: start.to_owned(),
      parents,
    }
  }

  fn reachable<'a>(&'a self) -> impl Iterator<Item = (i32, &'a str)> {
    self
      .parents
      .iter()
      .map(|(dest, (weight, _))| (*weight, dest.as_str()))
  }

  fn path_to(&self, dest: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut current = dest;

    loop {
      if current == self.start {
        break;
      }

      paths.push(current.to_owned());

      current = self.parents.get(current).unwrap().1.as_str();
    }

    paths.reverse();
    paths
  }
}

fn current_pressure(opened: &HashMap<String, i32>) -> i32 {
  opened.values().sum()
}

fn solve1(data: &HashMap<String, Valve>) -> i32 {
  let mut opened = HashMap::new();
  let mut max_pressure = 0;
  let mut current = "AA".to_owned();
  let mut minute = 0;

  while minute < 30 {
    let p1 = Paths::new(data, &current);

    if let Some((_, next)) = next_to_open(data, &current, &p1, &opened, minute) {
      println!("  next to open: {}", next);
      let mut p = p1.path_to(&next);

      for _ in &p {
        minute += 1;
        max_pressure += current_pressure(&opened);
        println!("MIN={:2} pressure: {}", minute, max_pressure);
      }

      opened.insert(next.to_owned(), data.get(&next).unwrap().rate);
      current = p.pop().unwrap();
    } else {
      println!("  nothing to do, everything is open");
    }

    minute += 1;
    max_pressure += current_pressure(&opened);
    println!("MIN={:2} pressure: {}", minute, max_pressure);
  }

  println!("{} vs {}", opened.len(), data.len());

  max_pressure
}
