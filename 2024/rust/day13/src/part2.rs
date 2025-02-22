use miette::miette;

use crate::parsers::parse;
use crate::types::{ClawMachine, Prize};

const SCALE_FACTOR: i64 = 10_000_000_000_000;

fn count_tokens(machine: &ClawMachine) -> Option<i64> {
  /*
   * Thanks to @UncleScientist for this solution
   *
   * m = number of times button_a is pushed
   * n = number of times button_b is pushed
   *
   * Expressing how we reach the prize...
   * p.x = a.x * m + b.x * n
   * p.y = a.y * m + b.y * n
   *
   * So ...
   * m = (p.x - b.x * n) / a.x
   * or
   * m = (p.y - b.y * n) / a.y
   *
   * In other words...
   * (p.x - b.x * n)    (p.y - b.y * n)
   * --------------  =   -------------
   *       a.x                a.y
   *
   * a.y * (p.x - b.x * n) = a.x * (p.y - b.y * n)
   *
   * a.y * p.x - a.y * b.x  * n = a.x * p.y - a.x * b.y * n
   *
   * a.x * b.y * n - a.y * b.x * n = a.x * p.y - a.y * p.x
   *
   * n * (a.x * b.y - a.y * b.x) = a.x * p.y - a.y * p.x
   *
   *      a.x * p.y - a.y * p.x
   * n =  ---------------------
   *      a.x * b.y - a.y * b.x
   *
   * m = (p.y - b.y * n) / a.y
   *
   */
  let button_a = machine.a;
  let button_b = machine.b;
  let prize = Prize::new(
    machine.prize.x + SCALE_FACTOR,
    machine.prize.y + SCALE_FACTOR,
  );

  let n = (button_a.x * prize.y - button_a.y * prize.x)
    / (button_a.x * button_b.y - button_a.y * button_b.x);

  let m = (prize.y - button_b.y * n) / button_a.y;

  if Prize::new(
    button_a.x * m + button_b.x * n,
    button_a.y * m + button_b.y * n,
  ) == prize
  {
    // Pressing button_a costs 3x more than pressing button_b
    Some(3 * m + n)
  } else {
    None
  }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let (_remaining_input, machines) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
  let sum: i64 = machines
    .iter()
    .filter_map(|machine| count_tokens(&machine))
    .sum();
  Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    // We don't have a test result provided for part 2!!
    assert_eq!("480", process(input)?);
    Ok(())
  }
}
