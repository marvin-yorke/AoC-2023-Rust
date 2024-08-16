/*

*/

use Almanac::Range;

mod Almanac {
  pub struct Range {
    pub src_start: u32,
    pub dst_start: u32,
    pub len: u32
  }

  impl Range {
    /// Checks if given number falls into current range.
    /// - returns: offset if range contains the given number, otherwise the number itself.
    pub fn contains(&self, num: u32) -> Option<u32> {
      if num < self.src_start {
        return None
      }

      let offset = num - self.src_start;
      if offset >= self.len {
        None
      } else {
        Some(offset)
      }
    }

    /// Find a mapping between source and destination ranges corresponding to `num`.
    pub fn map(&self, num: u32) -> Option<u32> {
      if let Some(offset) = self.contains(num) {
        Some(self.dst_start + offset)
      } else {
        None
      }
    }
  }
}

pub fn day05_a(lines: &[&str]) -> u32 {
  let seeds: Vec<u32> = lines[0]
    .split_once(": ")
    .map(|(_, l)| {
      l.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
    })
    .expect("Failed to parse seeds list");

  let sections = lines[1..]
    .split(|&l| l.trim().is_empty())
    .map(parse_section);

  seeds.iter()
    .inspect(|seed| print!("Seed {seed}"))
    .map(|&seed| {
      sections.clone().scan(seed, |num, sec| {
        *num = follow_section(*num, &sec);
        Some(*num)
      })
      .skip(1)
      .inspect(|pos| print!(" -> {pos}"))
      .last()
      .expect("No location for seed {seed}")
    })
    .inspect(|_| println!(""))
    .min()
    .expect("No location for seed {seed}")
}

pub fn day05_b(lines: &[&str]) -> u32 {
  0
}

fn parse_section(lines: &[&str]) -> Vec<Almanac::Range> {
  lines.iter()
    .skip(1) // skip header
    .map(|&l| {
      let mut values = l.split_whitespace().filter_map(|s| s.parse::<u32>().ok());

      let dst_start = values.next().unwrap();
      let src_start = values.next().unwrap();
      let len = values.next().unwrap();

      Almanac::Range { src_start, dst_start, len }
    })
    .collect()
}

fn follow_section(seed: u32, section: &Vec<Almanac::Range>) -> u32 {
  section.iter()
    .filter_map(|range| range.map(seed))
    .take(1)
    .next()
    .unwrap_or(seed)
}

#[cfg(test)]
mod tests {
  use super::Almanac;

  #[test]
  fn test_contains() {
    assert_eq!(Almanac::Range { src_start: 50, dst_start: 0, len: 2 }.contains(50), Some(0));
    assert_eq!(Almanac::Range { src_start: 50, dst_start: 0, len: 2 }.contains(51), Some(1));
    assert_eq!(Almanac::Range { src_start: 50, dst_start: 0, len: 2 }.contains(52), None);
    assert_eq!(Almanac::Range { src_start: 50, dst_start: 0, len: 2 }.contains(40), None);
  }

  #[test]
  fn test_map() {
    assert_eq!(Almanac::Range { src_start: 98, dst_start: 50, len: 2 }.map(98), 50);
    assert_eq!(Almanac::Range { src_start: 50, dst_start: 52, len: 48 }.map(53), 55);
    assert_eq!(Almanac::Range { src_start: 98, dst_start: 50, len: 2 }.map(10), 10);
  }
}
