const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("./input.txt");

fn main() {
  let sample = parse(SAMPLE);
  let input = parse(INPUT);

  println!("sample 1: {:?}", solve1(&sample));
  println!("input 1: {:?}", solve1(&input));

  println!("sample 2");
  solve2(&sample);

  println!("input 2");
  solve2(&input);
}

#[derive(Debug)]
enum OpCode {
  NoOp,
  AddX(i32),
}

fn parse(input: &str) -> Vec<OpCode> {
  input
    .lines()
    .map(|line| {
      let mut line = line.split_whitespace();
      match line.next().unwrap() {
        "noop" => OpCode::NoOp,
        _ => OpCode::AddX(line.next().unwrap().parse().unwrap()),
      }
    })
    .collect()
}

fn solve1(data: &Vec<OpCode>) -> i32 {
  data
    .iter()
    .fold((1, 1, 0), |(cycle, x_reg, mut strength), oc| {
      for k in 0..(if matches!(oc, OpCode::NoOp) { 1 } else { 2 }) {
        let cycle = cycle + k;

        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
          strength += cycle * x_reg
        }
      }

      match oc {
        OpCode::NoOp => (cycle + 1, x_reg, strength),
        OpCode::AddX(a) => (cycle + 2, x_reg + a, strength),
      }
    })
    .2
}

fn solve2(data: &Vec<OpCode>) {
  let mut cycle = 0;
  let mut x_reg = 1;

  for oc in data {
    for k in 0..(if matches!(oc, OpCode::NoOp) { 1 } else { 2 }) {
      if [40, 80, 120, 160, 200, 240].contains(&cycle) {
        println!("");
      }

      cycle += 1;

      let x_reg_mod = x_reg % 40;
      if [x_reg_mod - 1, x_reg_mod, x_reg_mod + 1].contains(&(cycle % 40)) {
        print!("#");
      } else {
        print!(".");
      }

      match oc {
        OpCode::AddX(a) if k == 0 => {
          x_reg += a;
        }

        _ => (),
      }
    }
  }
}
