use num::integer::lcm;

const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("./input.txt");

fn main() {
  let sample = parse(SAMPLE);
  let input = parse(INPUT);

  println!("sample 1: {:#?}", solve(20, 3, &sample));
  println!("input 1: {:#?}", solve(20, 3, &input));

  println!("sample 1: {:#?}", solve(10000, 1, &sample));
  println!("input 1: {:#?}", solve(10000, 1, &input));
}

#[derive(Clone, Debug)]
struct Monkey {
  items: Vec<u64>,
  op: Op,
  divisible_by: u64,
  success_monkey: usize,
  failure_monkey: usize,
  inspections: u64,
}

#[derive(Clone, Debug)]
enum Op {
  OldOld,
  OldAdd(u64),
  OldMul(u64),
}

impl Op {
  fn bias_worry_level(&self, worry_level: u64) -> u64 {
    match self {
      Op::OldOld => worry_level * worry_level,
      Op::OldAdd(a) => worry_level + a,
      Op::OldMul(a) => worry_level * a,
    }
  }
}

fn parse(input: &str) -> Vec<Monkey> {
  input
    .split("\n\n")
    .map(|monkey| {
      let mut lines = monkey.lines().skip(1).map(str::trim);

      let items = lines.next().unwrap();
      let items = items["Starting items: ".len()..]
        .split(", ")
        .flat_map(|item| item.parse())
        .collect();

      let op = lines.next().unwrap();
      let mut op = op["Operation: new = old ".len()..].split_whitespace();
      let op = match op.next().unwrap() {
        "+" => Op::OldAdd(op.next().unwrap().parse().unwrap()),
        "*" => {
          let right = op.next().unwrap();
          if right == "old" {
            Op::OldOld
          } else {
            Op::OldMul(right.parse().unwrap())
          }
        }
        _ => unreachable!(),
      };

      let divisible_by = lines.next().unwrap()["Test: divisible by ".len()..]
        .parse()
        .unwrap();

      let success_monkey = lines.next().unwrap()["If true: throw to monkey ".len()..]
        .parse()
        .unwrap();
      let failure_monkey = lines.next().unwrap()["If false: throw to monkey ".len()..]
        .parse()
        .unwrap();

      Monkey {
        items,
        op,
        divisible_by,
        success_monkey,
        failure_monkey,
        inspections: 0,
      }
    })
    .collect()
}

fn take_rounds(rounds: usize, common_divisor: u64, monkeys: &mut Vec<Monkey>) {
  let mut item_buffer = Vec::new();
  let modulo = find_modulo(monkeys);

  for _ in 0..rounds {
    for monkey_i in 0..monkeys.len() {
      item_buffer.clear();
      item_buffer.extend_from_slice(&monkeys[monkey_i].items[..]);

      for &item in &item_buffer {
        let worry = monkeys[monkey_i].op.bias_worry_level(item) / common_divisor;

        if worry % monkeys[monkey_i].divisible_by == 0 {
          let i = monkeys[monkey_i].success_monkey;
          monkeys[i].items.push(worry % modulo);
        } else {
          let i = monkeys[monkey_i].failure_monkey;
          monkeys[i].items.push(worry % modulo);
        }
      }

      let monkey = &mut monkeys[monkey_i];
      monkey.items.clear();
      monkey.inspections += item_buffer.len() as u64;
    }
  }
}

fn find_modulo(monkeys: &Vec<Monkey>) -> u64 {
  monkeys
    .iter()
    .map(|monkey| monkey.divisible_by)
    .reduce(lcm)
    .unwrap()
}

fn solve(rounds: usize, common_divisor: u64, monkeys: &Vec<Monkey>) -> u64 {
  let mut monkeys = monkeys.clone();
  take_rounds(rounds, common_divisor, &mut monkeys);

  monkeys.sort_by_key(|monkey| monkey.inspections);
  monkeys
    .iter()
    .rev()
    .take(2)
    .map(|monkey| monkey.inspections)
    .product::<u64>() as _
}
