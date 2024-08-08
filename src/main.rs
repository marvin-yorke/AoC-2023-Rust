use std::{fs::File, io::{self, Read}};
mod solutions;

fn main() -> io::Result<()> {
  let mut file = File::open("../input/03/input.txt")?;
  let mut content = String::new();

  file.read_to_string(&mut content)?;

  let lines: Vec<&str> = content
    .trim()
    .split("\n")
    .collect();

  let result: u32 = solutions::day03::day03_a(&lines);
  println!("{result}");

  Ok(())
}
