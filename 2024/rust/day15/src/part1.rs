use glam::I64Vec2;

use crate::types::{Moves, Warehouse};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let (mut warehouse, moves) = parse(input);
  for robot_move in moves {
    let new_position = warehouse.robot + robot_move;
    if warehouse.try_move(&new_position, &robot_move) {
      warehouse.robot = new_position;
    }
  }
  Ok(warehouse.box_score().to_string())
}

fn parse(input: &str) -> (Warehouse, Moves) {
  let (warehouse, moves) = input.split_once("\n\n").unwrap_or_default();
  let warehouse = Warehouse::from(warehouse);
  let moves: Moves = moves
    .lines()
    .flat_map(|line| {
      line
        .chars()
        .map(|ch| match ch {
          '>' => I64Vec2::Y, // Y is col
          'v' => I64Vec2::X, // X is row
          '<' => I64Vec2::NEG_Y,
          '^' => I64Vec2::NEG_X,
          _ => unreachable!(),
        })
        .collect::<Moves>()
    })
    .collect();
  (warehouse, moves)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    assert_eq!("2028", process(input)?);
    Ok(())
  }
}
