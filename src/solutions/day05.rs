/*

*/

mod almanac {
  use std::ops::Range;

  #[derive(Debug)]
  pub struct Entry {
    pub src_start: u64,
    pub dst_start: u64,
    pub len: u64
  }

  pub type Map = Vec<Entry>;

  #[derive(Debug)]
  pub struct Almanac {
    pub maps: Vec<Map>
  }

  impl Almanac {
    pub fn from_input(lines: &[&str]) -> Self {
      let sections = lines
        .split(|&l| l.trim().is_empty())
        .skip(1)
        .map(|lines| {
          lines.iter()
            .skip(1) // skip header
            .map(|&l| {
              let mut values = l.split_whitespace()
                .map(|s| s.parse().expect("Failed to parse section item"));

              let dst_start = values.next().unwrap();
              let src_start = values.next().unwrap();
              let len = values.next().unwrap();

              Entry { src_start, dst_start, len }
            })
            .collect()
        })
        .collect();

      Almanac { maps: sections }
    }
  }

  impl Entry {
    pub fn src_range(&self) -> Range<u64> {
      Range { start: self.src_start, end: self.src_start + self.len }
    }

    /// Translate `range` from source to destination.
    pub fn translate(&self, range: Range<u64>) -> Range<u64> {
      Range {
        start: range.start - self.src_start + self.dst_start,
        end: range.end - self.src_start + self.dst_start
      }
    }
  }
}

use std::ops::Range;
use std::cmp::{min, max};
use almanac::Almanac;

pub fn day05_a(lines: &[&str]) -> u32 {
  let (seeds_line, almanac_lines) = lines
    .split_first()
    .expect("Failed to parse input");

  let seeds: Vec<Range<u64>> = seeds_line
    .split_once(": ") // skip header
    .map(|(_, line)| {
      line.split_whitespace()
        .map(|s| {
          let n = s.parse()
            .expect("Failed to parse seeds");

          Range { start: n, end: n + 1 }
        })
        .collect()
    })
    .expect("Failed to parse seeds list");

  let Almanac { maps } = Almanac::from_input(almanac_lines);

  maps.into_iter()
    .fold(seeds.clone(), |seeds, map| follow_map(&seeds, &map))
    .into_iter()
    .map(|r| r.start)
    .min()
    .expect("Failed to get min value") as u32
}

pub fn day05_b(lines: &[&str]) -> u32 {
  let (seeds_line, almanac_lines) = lines
    .split_first()
    .expect("Failed to parse input");

  let seeds: Vec<Range<u64>> = seeds_line
    .split_once(": ") // skip header
    .map(|(_, l)| {
      l.split_whitespace()
        .map(|s| s.parse().expect("Failed to parse seeds"))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|pair| Range { start: pair[0], end: pair[1] + pair[0] })
        .collect()
    })
    .expect("Failed to parse seeds list");

  let Almanac { maps } = Almanac::from_input(almanac_lines);

  maps.into_iter()
    .fold(seeds.clone(), |seeds, map| follow_map(&seeds, &map))
    .into_iter()
    .map(|r| r.start)
    .min()
    .expect("Failed to get min value") as u32
}

#[derive(Debug)]
struct RangeIntersection {
  before: Option<Range<u64>>,
  result: Option<Range<u64>>,
  after: Option<Range<u64>>
}

fn intersect(seed: &Range<u64>, src: &Range<u64>) -> RangeIntersection {
  let int_start: u64;
  let int_end: u64;

  let before: Option<Range<u64>>;

  if seed.start >= src.start {
    before = None;
    int_start = seed.start;
  } else {
    int_start = src.start;
    before = Some(
      Range {
        start: seed.start,
        end: min(src.start, seed.end)
      }
    );
  }

  let after: Option<Range<u64>>;

  if seed.end <= src.end {
    after = None;
    int_end = seed.end;
  } else {
    int_end = src.end;
    after = Some(
      Range {
        start: max(src.end, seed.start),
        end: seed.end
      }
    );
  }

  let int = if int_start < int_end {
    Some(Range { start: int_start, end: int_end })
  } else {
    None
  };

  RangeIntersection { before, result: int, after }
}

fn follow_map(seeds: &Vec<Range<u64>>, map: &almanac::Map) -> Vec<Range<u64>> {
  let mut res = Vec::new();
  let mut seeds = seeds.clone();

  while let Some(seed_range) = seeds.pop() {
    let mut found = false;

    for entry in map.into_iter() {
      let i = intersect(&seed_range, &entry.src_range());

      if let Some(intersection) = i.result {
        found = true;
        res.push(entry.translate(intersection));

        if let Some(before) = i.before {
          seeds.push(before)
        }

        if let Some(after) = i.after {
          seeds.push(after)
        }
      }
    }

    // none of the maps intersect with `seed_range`, pass it as is to hte next step
    if !found {
      res.push(seed_range);
    }
  }

  res
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_no_intersection() {
    let seed = 0..5;
    let src = 10..15;
    let result = intersect(&seed, &src);
    assert_eq!(result.before, Some(0..5));
    assert_eq!(result.result, None);
    assert_eq!(result.after, None);
  }

  #[test]
  fn test_complete_overlap() {
    let seed = 0..10;
    let src = 2..8;
    let result = intersect(&seed, &src);
    assert_eq!(result.before, Some(0..2));
    assert_eq!(result.result, Some(2..8));
    assert_eq!(result.after, Some(8..10));
  }

  #[test]
  fn test_partial_overlap_start() {
    let seed = 0..5;
    let src = 3..10;
    let result = intersect(&seed, &src);
    assert_eq!(result.before, Some(0..3));
    assert_eq!(result.result, Some(3..5));
    assert_eq!(result.after, None);
  }

  #[test]
  fn test_partial_overlap_end() {
    let seed = 5..10;
    let src = 0..7;
    let result = intersect(&seed, &src);
    assert_eq!(result.before, None);
    assert_eq!(result.result, Some(5..7));
    assert_eq!(result.after, Some(7..10));
  }

  #[test]
  fn test_seed_inside_src() {
    let seed = 3..7;
    let src = 0..10;
    let result = intersect(&seed, &src);
    assert_eq!(result.before, None);
    assert_eq!(result.result, Some(3..7));
    assert_eq!(result.after, None);
  }

  #[test]
  fn test_src_inside_seed() {
    let seed = 0..10;
    let src = 3..7;
    let result = intersect(&seed, &src);
    assert_eq!(result.before, Some(0..3));
    assert_eq!(result.result, Some(3..7));
    assert_eq!(result.after, Some(7..10));
  }

  #[test]
  fn test_equal_ranges() {
    let seed = 0..10;
    let src = 0..10;
    let result = intersect(&seed, &src);
    assert_eq!(result.before, None);
    assert_eq!(result.result, Some(0..10));
    assert_eq!(result.after, None);
  }

  #[test]
  fn test_adjacent_ranges() {
    let seed = 0..5;
    let src = 5..10;
    let result = intersect(&seed, &src);
    assert_eq!(result.before, Some(0..5));
    assert_eq!(result.result, None);
    assert_eq!(result.after, None);
  }
}
