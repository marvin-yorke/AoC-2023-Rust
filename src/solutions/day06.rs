/*

*/

pub fn day06_a(lines: &[&str]) -> usize {
  let races = parse_input_a(lines);

  races
    .into_iter()
    .map(|race| num_outcomes(&race))
    .product()
}

pub fn day06_b(lines: &[&str]) -> usize {
  let race = parse_input_b(lines);
  num_outcomes(&race)
}

#[derive(Debug)]
struct Race {
  time: usize,
  distance: usize
}

fn parse_input_a(lines: &[&str]) -> Vec<Race> {
  let time_line = lines[0];
  let distance_line = lines[1];

  let times = time_line
    .trim_start_matches(|c: char| {
      c.is_ascii_digit() == false
    })
    .split_ascii_whitespace()
    .map(|s| s.parse::<usize>().expect("not a number"));

  let distances = distance_line
    .trim_start_matches(|c: char| {
      c.is_ascii_digit() == false
    })
    .split_ascii_whitespace()
    .map(|s| s.parse::<usize>().expect("not a number"));

  times.zip(distances)
    .map(|(time, distance)| Race { time, distance })
    .collect()
}

fn parse_input_b(lines: &[&str]) -> Race {
  let time_line = lines[0];
  let distance_line = lines[1];

  let time = time_line
    .trim_start_matches(|c: char| {
      c.is_ascii_digit() == false
    })
    .split_ascii_whitespace()
    .fold(0, |acc, s| {
      (acc * 10usize.pow(s.len() as u32)) + s.parse::<usize>().expect("not a number")
    });

  let distance = distance_line
    .trim_start_matches(|c: char| {
      c.is_ascii_digit() == false
    })
    .split_ascii_whitespace()
    .fold(0, |acc, s| {
      acc * 10usize.pow(s.len() as u32) + s.parse::<usize>().expect("not a number")
    });

  Race { time, distance }
}

fn is_winning(press_time: usize, race_time: usize, record_distance: usize) -> bool {
  let speed = press_time;
  let go_time = race_time - press_time;
  let actual_disatance = speed * go_time;

  actual_disatance > record_distance
}

fn num_outcomes(race: &Race) -> usize {
  let half_loosing = (1..=race.time / 2).into_iter()
    .take_while(|&t| !is_winning(t, race.time, race.distance))
    .count();

  race.time - 2 * half_loosing - 1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cant_win() {
    assert!(is_winning(0, 7, 9) == false);
    assert!(is_winning(3, 7, 9));
  }

}
