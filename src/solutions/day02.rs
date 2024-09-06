/*

*/

use std::cmp::max;

#[derive(Debug)]
struct Game {
  id: usize,
  red: usize,
  green: usize,
  blue: usize
}

impl Game {
  fn from_string(s: &str) -> Self {
    let (game, rounds) = s.split_once(": ").unwrap();

    let id = game.strip_prefix("Game ")
      .and_then(|num_str| num_str.parse::<usize>().ok()).unwrap();

    fn parse_round(round: &str) -> (usize, usize, usize) {
      let initial: (usize, usize, usize) = (0, 0, 0);

      round.split(", ")
        .map(|s|
          s.split_once(" ").and_then(|(num, color)|
            num.parse::<usize>().map(|num| (num, color)).ok()
          )
        )
        .fold(initial, |acc, color| {
          match color {
            Some((num, "red")) => (acc.0 + num, acc.1, acc.2),
            Some((num, "green")) => (acc.0, acc.1 + num, acc.2),
            Some((num, "blue")) => (acc.0, acc.1, acc.2 + num),
            _ => acc
          }
        })
    }

    let colors = rounds.split("; ")
      .map(&parse_round)
      .reduce(|acc, e|
        (
          max(acc.0, e.0),
          max(acc.1, e.1),
          max(acc.2, e.2)
        )
      ).unwrap();

    Self {
      id,
      red: colors.0,
      green: colors.1,
      blue: colors.2
    }
  }

  fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
    self.red <= red && self.green <= green && self.blue <= blue
  }
}

fn day02_a(lines: &[&str]) -> usize {
  lines.iter()
    .map(|&line| Game::from_string(line))
    .filter_map(|game|
      if game.is_possible(12, 13, 14) {
        Some(game.id)
      } else {
        None
      }
    )
    .sum()
}

fn day02_b(lines: &[&str]) -> usize {
  lines.iter()
    .map(|&line| Game::from_string(line))
    .map(|game| game.red * game.green * game.blue)
    .sum()
}
