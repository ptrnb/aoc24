use day16::part1::process;
use miette::Context;

fn main() -> miette::Result<()> {
  tracing_subscriber::fmt::init();
  let file = include_str!("../../input1.txt");
  let result = process(file).context("process part1")?;
  println!("{}", result);
  Ok(())
}
