use day12::part1pg::process;
use miette::Context;

fn main() -> miette::Result<()> {
  tracing_subscriber::fmt::init();
  let file = include_str!("../../input1.txt");
  let result = process(file).context("process part1pg")?;
  println!("{}", result);
  Ok(())
}
