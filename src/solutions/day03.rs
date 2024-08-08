/*

*/

struct Candidate {
  val: u32,
  adjacent_symbols: u32
}

pub fn day03_a(lines: &[&str]) -> u32 {
  let line_len = lines.first().unwrap().len();

  let extended = vec![
    vec![String::from_utf8(std::iter::repeat(b'.').take(line_len).collect()).unwrap()],
    lines.iter().map(|&line| {
      format!(".{}.", line.to_string())
    }).collect(),
    vec![String::from_utf8(std::iter::repeat(b'.').take(line_len).collect()).unwrap()],
  ].concat();

  let mut sum: u32 = 0;

  fn check_adjacency(top: char, mid: char, bottom: char) -> u32 {
    let is_symbol = |char: char| if char.is_digit(10) == false && char != '.' { 1 } else { 0 };
    is_symbol(top) + is_symbol(mid) + is_symbol(bottom)
  }

  for row in 1..(lines.len() + 1) {
    let mut candidate: Option<Candidate> = None;

    let line = extended[row].chars().collect::<Vec<char>>();
    println!("{}", extended[row]);

    for col in 1..line_len + 2 { //check extended line length to make sure last number on the line is added
      let char = line[col];

      candidate = match (char.to_digit(10), candidate) {
        // has digit and has existing candidate
        (Some(digit), Some(Candidate { val, mut adjacent_symbols })) => {
          // check adjacency on top/bottom lines
          let top = extended[row-1].chars().nth(col).unwrap_or('.');
          let bottom = extended[row+1].chars().nth(col).unwrap_or('.');

          adjacent_symbols += check_adjacency(top, char, bottom);

          Some(Candidate {
            val: val * 10 + digit,
            adjacent_symbols
          })
        },

        // has digit but no existing candidate
        (Some(digit), None) => {
          // check left adjacency
          let left_adjacent = {
            let top = extended[row-1].chars().nth(col-1).unwrap_or('.');
            let mid = extended[row].chars().nth(col-1).unwrap_or('.');
            let bottom = extended[row+1].chars().nth(col-1).unwrap_or('.');

            check_adjacency(top, mid, bottom)
          };

          let this_adjacent = {
            let top = extended[row-1].chars().nth(col).unwrap_or('.');
            let bottom = extended[row+1].chars().nth(col).unwrap_or('.');

            check_adjacency(top, char, bottom)
          };

          Some(Candidate {
            val: digit,
            adjacent_symbols: left_adjacent + this_adjacent
          })
        },

        // no digit but has existng candidate
        (None, Some(Candidate { val, mut adjacent_symbols })) => {
          // check right adjacency

          let top = extended[row-1].chars().nth(col).unwrap_or('.');
          let mid = extended[row].chars().nth(col).unwrap_or('.');
          let bottom = extended[row+1].chars().nth(col).unwrap_or('.');

          adjacent_symbols += check_adjacency(top, mid, bottom);

          if adjacent_symbols > 1 {
            println!("{adjacent_symbols}");
          }

          sum += val * adjacent_symbols;

          None
        },

        // no digit and no candidate
        (None, None) => None
      };
    }
  }

  sum
}
