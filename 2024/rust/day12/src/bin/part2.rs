use day12::part2::process;
use miette::Context;

fn main() -> miette::Result<()> {
  tracing_subscriber::fmt::init();
  let file = include_str!("../../input2.txt"); 
  let result = process(file).context("process part2")?;
  println!("{}", result);
  Ok(())
}
