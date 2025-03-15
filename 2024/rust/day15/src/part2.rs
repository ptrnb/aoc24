use crate::types::{Moves, WideWarehouse};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let (moves, mut wide_warehouse) = parse(input);
  for robot_direction in moves {
    let new_position = wide_warehouse.robot + robot_direction;
    if wide_warehouse.try_big_move(new_position, robot_direction) {
      wide_warehouse.robot = new_position;
    }
  }
  Ok(wide_warehouse.box_score().to_string())
}

fn parse(input: &str) -> (Moves, WideWarehouse) {
  let (wide_warehouse, moves) = input.split_once("\n\n").unwrap_or_default();
  let wide_warehouse = WideWarehouse::from(wide_warehouse);
  let moves: Moves = moves
    .lines()
    .flat_map(|line| {
      line
        .chars()
        .map(|ch| match ch {
          '>' => WideWarehouse::RIGHT,
          'v' => WideWarehouse::DOWN,
          '<' => WideWarehouse::LEFT,
          '^' => WideWarehouse::UP,
          _ => unreachable!(),
        })
        .collect::<Moves>()
    })
    .collect();
  (moves, wide_warehouse)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    assert_eq!("9021", process(input)?);
    Ok(())
  }
}
