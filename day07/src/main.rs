use std::collections::HashMap;

const SAMPLE: &'static str = include_str!("./sample.txt");
const INPUT: &'static str = include_str!("./input.txt");

const MEM_TOTAL: u32 = 70000000;
const MEM_UPDATE: u32 = 30000000;

fn main() {
  let sample = process(SAMPLE.lines());
  let input = process(INPUT.lines());

  println!("sample 1: {:#?}", solve1(&sample));
  println!("part 1: {:#?}", solve1(&input));

  println!("sample 2: {:#?}", solve2(&sample));
  println!("part 2: {:#?}", solve2(&input));
}

fn mk_path(cwd: &Vec<(&str, u32)>) -> String {
  cwd
    .iter()
    .map(|(name, _)| *name)
    .collect::<Vec<_>>()
    .join("/")
}

fn process<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<String, u32> {
  let mut cwd = Vec::new();
  let mut sizes: HashMap<String, u32> = HashMap::new();

  let mut lines = lines.peekable();
  while let Some(line) = lines.next() {
    if line.starts_with("$ cd ") {
      match &line["$ cd ".len()..] {
        "/" => {
          cwd.clear();
          cwd.push(("", 0));
        }

        ".." => {
          let path = mk_path(&cwd);
          if let Some((_, weight)) = cwd.pop() {
            sizes.insert(path, weight);

            // and add it the new cwd
            if let Some((_, new_weight)) = cwd.last_mut() {
              *new_weight += weight;
            }
          }
        }

        dir => {
          cwd.push((dir, 0));
        }
      }
    } else if line == "$ ls" {
      while let Some(line) = lines.peek() {
        if line.starts_with("$") {
          break;
        }

        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let left = parts.next().unwrap();

        if left != "dir" {
          let weight: u32 = left.parse().unwrap();
          if let Some(cwd) = cwd.last_mut() {
            cwd.1 += weight;
          }
        }
      }
    }
  }

  // finish computing the sizes
  while cwd.len() > 1 {
    let path = mk_path(&cwd);
    if let Some((_, weight)) = cwd.pop() {
      sizes.insert(path, weight);

      // and add it the new cwd
      if let Some((_, new_weight)) = cwd.last_mut() {
        *new_weight += weight;
      }
    }
  }

  sizes.insert("/".to_string(), cwd.pop().unwrap().1);
  sizes
}

fn solve1(data: &HashMap<String, u32>) -> u32 {
  data
    .iter()
    .filter_map(|(_, weight)| {
      if *weight <= 100000 {
        Some(weight)
      } else {
        None
      }
    })
    .sum()
}

fn solve2(data: &HashMap<String, u32>) -> u32 {
  let unused = MEM_TOTAL - data.get("/").unwrap();
  data
    .iter()
    .filter_map(|(_, &weight)| {
      if weight + unused >= MEM_UPDATE {
        Some(weight)
      } else {
        None
      }
    })
    .min()
    .unwrap()
}
