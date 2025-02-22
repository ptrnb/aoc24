use glam::I64Vec2;
use miette::miette;
use pathfinding::prelude::dijkstra;

use crate::parsers::parse;

const A_COST: u32 = 3;
const B_COST: u32 = 1;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let (_, machines) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
  let sum: u32 = machines
    .iter()
    .map(|machine| {
      let start_node = I64Vec2::ZERO;
      let result = dijkstra(
        &start_node,
        |position| {
          if position.x > machine.prize.x || position.y > machine.prize.y {
            vec![]
          } else {
            vec![
              (position + machine.a, A_COST),
              (position + machine.b, B_COST),
            ]
          }
        },
        |&goal| goal == machine.prize,
      );
      result.map(|(_path, cost)| cost)
    })
    .flatten()
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
    assert_eq!("480", process(input)?);
    Ok(())
  }
}
