/*

*/

use std::collections::HashMap;

#[derive(Debug,Hash,PartialEq,Eq,Clone,Copy)]
struct Coord {
  row: usize,
  col: usize
}

enum Dir {
  TopLeft,
  Left, Right,
  BottomLeft,
}

impl Coord {
  fn offset(&self, dir: Dir) -> Self {
    use Dir::*;

    Self {
      row: match dir {
        TopLeft => self.row - 1,
        BottomLeft => self.row + 1,
        Left | Right => self.row
      },
      col: match dir {
        TopLeft | Left | BottomLeft => self.col - 1,
        Right => self.col + 1
      }
    }
  }
}

struct Candidate {
  val: u32,
  adjacent_symbols: Vec<Coord>
}

struct Gear {
  sym: char,
  ratios: Vec<u32>
}

type Cache = HashMap<Coord, Gear>;

pub fn day03_a(lines: &[&str]) -> u32 {
  let extended = extend(lines);
  let symbols = parse(lines, &extended);

  symbols.values()
    .flat_map(|g| &g.ratios)
    .sum()
}

pub fn day03_b(lines: &[&str]) -> u32 {
  let extended = extend(lines);
  let symbols = parse(lines, &extended);

  symbols.values()
    .filter(|g| g.sym == '*' && g.ratios.len() == 2)
    .map(|g| g.ratios[..2].iter().product::<u32>())
    .sum()
}

/// Pad input with extra empty lines top/bottom and empty chars left/right for easier indexing.
///
/// Input: 1
///
/// Output: ...
///         .1.
///         ...
fn extend(lines: &[&str]) -> Vec<String> {
  let line_len = lines.first().unwrap().len();
  let pad_line = || std::iter::repeat(b'.').take(line_len);

  vec![
    vec![String::from_utf8(pad_line().collect()).unwrap()],
    lines.iter().map(|&line| { format!(".{}.", line) }).collect(),
    vec![String::from_utf8(pad_line().collect()).unwrap()],
  ].concat()
}

fn parse(lines: &[&str], extended: &[String]) -> Cache {
    let mut symbols: Cache = Cache::new();

    for row in 1..(lines.len() + 1) {
        let mut candidate: Option<Candidate> = None;

        for (col, char) in extended[row].char_indices().skip(1) {
          let coord = Coord { row, col };

          candidate = match (char.to_digit(10), candidate) {
            // has digit but no existing candidate
            (Some(digit), None) => {
              Some(Candidate {
                val: digit,
                adjacent_symbols: check_adjacency_left(&mut symbols, &extended, &coord)
              })
            },

            // has digit and has existing candidate
            (Some(digit), Some(Candidate { val, mut adjacent_symbols })) => {
              adjacent_symbols.append(
                &mut check_adjacency_left(&mut symbols, &extended, &coord
              ));

              Some(Candidate {
                val: val * 10 + digit,
                adjacent_symbols
              })
            },

            // no digit
            (None, candidate) => {
              if char != '.' {
                symbols.entry(coord)
                  .or_insert(Gear {
                    sym: char,
                    ratios: Vec::new()
                  });
              }

              if let Some(Candidate { val, mut adjacent_symbols }) = candidate {
                adjacent_symbols.append(
                  &mut check_adjacency_left(&mut symbols, &extended, &coord)
                );

                // Offset coord to the right to check current position with `_left` access.
                adjacent_symbols.append(
                  &mut check_adjacency_left(&mut symbols, &extended, &coord.offset(Dir::Right))
                );

                register_gear_ratio(&mut symbols, adjacent_symbols, val);
              }

              None
            }
          };
        }
      }

    symbols
}

/// Checks adjacent symbols to the left of `coord` position
/// and return their coordinates.
fn check_adjacency_left(symbols: &mut Cache, input: &[String], coord: &Coord) -> Vec<Coord> {
  let mut adjacent_symbols = Vec::<Coord>::new();

  {
    let top_left = coord.offset(Dir::TopLeft);

    if symbols.contains_key(&top_left) {
      adjacent_symbols.push(top_left)
    }
  }

  {
    let left = coord.offset(Dir::Left);

    if symbols.contains_key(&left) {
      adjacent_symbols.push(left)
    }
  }

  {
    let bottom_left = coord.offset(Dir::BottomLeft);

    let row = &input[bottom_left.row];
    let char = row.chars()
      .nth(bottom_left.col)
      .filter(|c| !(c.is_digit(10) || *c == '.'));

    if let Some(sym) = char {
      symbols.entry(bottom_left).or_insert(Gear { sym, ratios: Vec::new() });
      adjacent_symbols.push(bottom_left);
    }
  }

  adjacent_symbols
}

/// Add gear ratio value to symbols at adjacent coordinates.
fn register_gear_ratio(symbols: &mut Cache, adjacent_symbols: Vec<Coord>, val: u32) {
  for coord in adjacent_symbols {
    symbols.entry(coord).and_modify(|tuple| {
      tuple.ratios.push(val);
    });
  }
}
