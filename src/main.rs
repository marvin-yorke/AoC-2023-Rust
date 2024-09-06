mod solutions;

fn main() {
  let content = include_str!("../input/06/input.txt");

  let lines: Vec<&str> = content
    .trim()
    .split("\n")
    .collect();

  let result: usize = solutions::day06::day06_a(&lines);
  println!("Part A result: {result}");

  let result: usize = solutions::day06::day06_b(&lines);
  println!("Part B result {result}");
}
