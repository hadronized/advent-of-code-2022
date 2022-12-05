use std::collections::VecDeque;

const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("./input.txt");

fn main() {
  let sample = parse(SAMPLE);
  let input = parse(INPUT);

  println!("sample 1 {}", solve1(&sample.0, &sample.1));
  println!("part 1 {}", solve1(&input.0, &input.1));

  println!("sample 2 {}", solve2(&sample.0, &sample.1));
  println!("part 2 {}", solve2(&input.0, &input.1));
}

type Crates = Vec<VecDeque<u8>>;
type Orders = Vec<(u32, u32, u32)>;

fn parse(input: &str) -> (Crates, Orders) {
  let mut split = input.split("\n\n");
  let mut crates = Vec::new();

  split
    .next()
    .unwrap()
    .lines()
    .filter(|line| line.len() >= 2 && !line.as_bytes()[1].is_ascii_digit()) // ensure it’s a crate line
    .for_each(|line | {
      let bytes = line.as_bytes();
      for i in 1 .. bytes.len() {
        let k = (i - 1) / 4;

        if k == crates.len() {
          crates.push(VecDeque::new());
        }

        let c = bytes[i];
        if c.is_ascii_uppercase() {
          crates[k].push_front(c);
        }
      }
    });

  let orders = split
    .next()
    .unwrap()
    .lines()
    .map(|line| {
      let mut orders =
        line
          .split_whitespace()
          .enumerate()
          .filter_map(|(i, x)| if i & 1 == 1 { Some(x) } else { None });

      let a = orders.next().unwrap();
      let b = orders.next().unwrap();
      let c = orders.next().unwrap();

      (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap())
    })
    .collect();

  (crates, orders)
}

fn solve1(crates: &Crates, orders: &Orders) -> String {
  let mut crates = crates.clone();

  for (quantity, from, to) in orders {
    let from = *from as usize - 1;
    let to = *to as usize - 1;

    for _ in 0..*quantity {
      let c = crates[from].pop_back().unwrap();
      crates[to].push_back(c);
    }
  }

  crates
    .into_iter()
    .flat_map(|crates| crates.into_iter().map(|c| c as char).last())
    .collect()
}

fn solve2(crates: &Crates, orders: &Orders) -> String {
  let mut crates = crates.clone();
  let mut tmp = VecDeque::new();

  for (quantity, from, to) in orders {
    let from = *from as usize - 1;
    let to = *to as usize - 1;

    for _ in 0..*quantity {
      let c = crates[from].pop_back().unwrap();
      tmp.push_front(c);
    }

    for c in &tmp {
      crates[to].push_back(*c);
    }

    tmp.clear();
  }

  crates
    .into_iter()
    .flat_map(|crates| crates.into_iter().map(|c| c as char).last())
    .collect()
}
