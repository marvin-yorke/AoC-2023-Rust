use std::{fs::File, io::{self, Read}};
mod solutions;

fn main() -> io::Result<()> {
  let mut file = File::open("../input/05/test.txt")?;
  // let mut file = File::open("../input/05/input.txt")?;
  let mut content = String::new();

  file.read_to_string(&mut content)?;

  let lines: Vec<&str> = content
    .trim()
    .split("\n")
    .collect();

  let result: u32 = solutions::day05::day05_a(&lines);
  println!("Part A result: {result}");

  let result: u32 = solutions::day05::day05_b(&lines);
  println!("Part B result {result}");

  Ok(())
}
