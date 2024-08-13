/*

*/

use std::collections::HashSet;

struct Card {
  winning_numbers: HashSet<u32>,
  numbers: HashSet<u32>
}

impl Card {
  fn parse(winning: &str, numbers: &str) -> Self {
    Self {
      winning_numbers: winning.split_whitespace()
        .map(|val| val.parse::<u32>().unwrap())
        .collect(),
      numbers: numbers.split_whitespace()
        .map(|val| val.parse::<u32>().unwrap())
        .collect()
    }
  }
}

fn bonus_points(line: &str) -> u32 {
  line
    .split_once(": ")
    .and_then(|(_, s)| s.split_once(" | "))
    .map(|(w, n)| Card::parse(w, n))
    .map_or(0, |c| c.winning_numbers.intersection(&c.numbers).count() as u32)
}

pub fn day04_a(lines: &[&str]) -> u32 {
  lines.into_iter()
    .map(|&l| bonus_points(l))
    .filter(|&p| p > 0)
    .map(|p| 2u32.pow(p - 1))
    .sum()
}

pub fn day04_b(lines: &[&str]) -> u32 {
  let count = lines.len();
  let mut stack: Vec<u32> = std::iter::repeat(1).take(count).collect();

  for (i, &line) in lines.into_iter().enumerate() {
    let won = bonus_points(line);

    let min = i + 1;
    let max = std::cmp::min(min + won as usize, count);

    for j in min..max {
      stack[j] += stack[i];
    }
  }

  stack.iter().sum()
}
