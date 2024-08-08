/*

*/

pub fn day01_a(lines: &[&str]) -> u32 {
  lines.into_iter().map(|&line| {
    let chars: Vec<u32> = line.chars()
      .filter_map(|char| char.to_digit(10))
      .collect();

    chars.first().unwrap() * 10 + chars.last().unwrap()
  })
  .sum()
}

pub fn day01_b(lines: &[&str]) -> u32 {
  let digits = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "0",    "1",   "2",   "3",     "4",    "5",    "6",   "7",     "8",     "9"
  ];

  let get_digit = |idx: usize| { u32::try_from(idx % 10).unwrap() };

  lines.iter().map(|&line| {
    let mut out = Vec::<u32>::new();

    for i in 0..line.len() {
      for (idx, &pattern) in digits.iter().enumerate() {
        if line[i..].starts_with(pattern) {
          out.push(get_digit(idx));
        }
      }
    }

    out.first().unwrap() * 10 + out.last().unwrap()
  })
  .sum()
}
