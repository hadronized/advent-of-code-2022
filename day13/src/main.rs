use std::{cmp::Ordering, str::from_utf8};

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

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
  Number(u32),
  Sub(Vec<Packet>),
}

impl Ord for Packet {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl PartialOrd for Packet {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match (self, other) {
      (Packet::Number(x), Packet::Number(y)) => x.partial_cmp(y),
      (Packet::Sub(p), Packet::Sub(q)) => Some(compare(&p, &q)),
      (Packet::Number(a), Packet::Sub(q)) => Some(compare(&[Packet::Number(*a)], &q)),
      (Packet::Sub(p), Packet::Number(b)) => Some(compare(&p, &[Packet::Number(*b)])),
    }
  }
}

impl Packet {
  fn parse(input: &str) -> Vec<Self> {
    Self::parse_rec(input.as_bytes()).0
  }

  fn parse_rec(input: &[u8]) -> (Vec<Self>, usize) {
    let mut packets = Vec::new();
    let mut i = 0;

    while i < input.len() {
      match input[i] {
        b'[' => {
          i += 1;
          let (p, read) = Packet::parse_rec(&input[i..]);
          i += read;

          packets.push(Packet::Sub(p));
        }

        b']' => {
          i += 1;
          break;
        }

        b',' => {
          i += 1;
        }

        n if n.is_ascii_digit() => {
          let mut j = i + 1;

          while j < input.len() && input[j].is_ascii_digit() {
            j += 1;
          }

          let number = from_utf8(&input[i..j]).unwrap().parse().unwrap();
          packets.push(Packet::Number(number));

          i = j;
        }

        _ => unreachable!(),
      }
    }

    (packets, i)
  }
}

fn parse(input: &str) -> Vec<[Vec<Packet>; 2]> {
  input
    .split("\n\n")
    .map(|lines| {
      let mut iter = lines.split_whitespace().map(Packet::parse);
      [iter.next().unwrap(), iter.next().unwrap()]
    })
    .collect()
}

fn compare(a: &[Packet], b: &[Packet]) -> Ordering {
  for ordering in a.iter().zip(b).map(|(a, b)| match (a, b) {
    (Packet::Number(x), Packet::Number(y)) => x.cmp(y),
    (Packet::Sub(p), Packet::Sub(q)) => compare(&p, &q),
    (Packet::Number(a), Packet::Sub(q)) => compare(&[Packet::Number(*a)], &q),
    (Packet::Sub(p), Packet::Number(b)) => compare(&p, &[Packet::Number(*b)]),
  }) {
    match ordering {
      Ordering::Equal => continue,
      _ => return ordering,
    }
  }

  a.len().cmp(&b.len())
}

fn solve1(data: &Vec<[Vec<Packet>; 2]>) -> u32 {
  data
    .iter()
    .enumerate()
    .filter(|(_, [a, b])| a <= b)
    .map(|(i, _)| i as u32 + 1)
    .sum()
}

fn solve2(data: &Vec<[Vec<Packet>; 2]>) -> u32 {
  let mut packets = data
    .iter()
    .flat_map(|pair| pair.into_iter())
    .cloned()
    .collect::<Vec<_>>();
  let d1 = Packet::parse("[[2]]");
  let d2 = Packet::parse("[[6]]");

  packets.push(d1.clone());
  packets.push(d2.clone());
  packets.sort();

  packets
    .iter()
    .enumerate()
    .filter(|(_, p)| **p == d1 || **p == d2)
    .map(|(i, _)| i as u32 + 1)
    .product()
}
