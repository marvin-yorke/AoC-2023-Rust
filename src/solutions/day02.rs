/*

*/

use std::cmp::max;

#[derive(Debug)]
struct Game {
  id: u32,
  red: u32,
  green: u32,
  blue: u32
}

impl Game {
  fn from_string(s: &str) -> Self {
    let (game, rounds) = s.split_once(": ").unwrap();

    let id = game.strip_prefix("Game ")
      .and_then(|num_str| num_str.parse::<u32>().ok()).unwrap();

    fn parse_round(round: &str) -> (u32, u32, u32) {
      let initial: (u32, u32, u32) = (0, 0, 0);

      round.split(", ")
        .map(|s|
          s.split_once(" ").and_then(|(num, color)|
            num.parse::<u32>().map(|num| (num, color)).ok()
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

  fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
    self.red <= red && self.green <= green && self.blue <= blue
  }
}

fn day02_a(lines: &[&str]) -> u32 {
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

fn day02_b(lines: &[&str]) -> u32 {
  lines.iter()
    .map(|&line| Game::from_string(line))
    .map(|game| game.red * game.green * game.blue)
    .sum()
}
