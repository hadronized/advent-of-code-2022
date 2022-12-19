use std::{
  cmp::Reverse,
  collections::{BTreeMap, HashMap, HashSet},
  iter::once,
};

const SAMPLE: &'static str = include_str!("./sample.txt");

fn main() {
  let sample = parse(SAMPLE);

  println!("sample part 1: {}", solve1(&sample));
}

#[derive(Debug)]
struct Valve {
  rate: u32,
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
  p: &Paths,
  opened: &HashMap<String, u32>,
  minute: i32,
) -> (u32, String) {
  let mut interesting: Vec<_> = p
    .parents
    .iter()
    .filter(|(name, _)| !opened.contains_key(*name) && data.get(*name).unwrap().rate > 0)
    .collect();

  println!("interesting: {}", interesting.len());
  interesting.sort_by_key(|(target, (weight, _))| {
    println!(
      "  potential of {}: {}, min:{}, weight:{}, rate:{}",
      target,
      data.get(*target).unwrap().rate as i32 * (30 - minute - *weight),
      minute,
      weight,
      data.get(*target).unwrap().rate
    );
    Reverse(data.get(*target).unwrap().rate as i32 * (30 - minute - *weight))
  });

  let candidate = interesting.first().unwrap().0.clone();

  (data.get(&candidate).unwrap().rate, candidate)
}

#[derive(Debug)]
struct Paths {
  start: String,
  parents: HashMap<String, (i32, String)>,
}

impl Paths {
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

fn paths(data: &HashMap<String, Valve>, start: &str) -> Paths {
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

fn current_pressure(opened: &HashMap<String, u32>) -> u32 {
  opened.values().sum()
}

fn solve1(data: &HashMap<String, Valve>) -> u32 {
  let mut opened = HashMap::new();
  let mut max_pressure = 0;
  let mut current = "AA".to_owned();
  let mut minute = 0;

  while minute < 30 && opened.len() != data.len() {
    let p = paths(data, &current);
    let (rate, name) = next_to_open(data, &p, &opened, minute);

    if current != name {
      for next in p.path_to(&name) {
        println!("MIN={:2} moving to {}", minute, name);
        current = next;
        max_pressure += current_pressure(&opened);
        minute += 1;
      }
    }

    println!("MIN={:2} opening {}", minute, name);
    opened.insert(name, rate);
    max_pressure += current_pressure(&opened);
    minute += 1;
  }

  println!("{} vs {}", opened.len(), data.len());

  max_pressure
}
