use crate::types::{Moves, Warehouse};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let (moves, mut warehouse) = parse(input);
  for robot_direction in moves {
    let new_position = warehouse.robot + robot_direction;
    if warehouse.try_move(&new_position, &robot_direction) {
      warehouse.robot = new_position;
    }
  }
  // println!("{}", &warehouse);
  Ok(warehouse.box_score().to_string())
}

fn parse(input: &str) -> (Moves, Warehouse) {
  let (warehouse, moves) = input.split_once("\n\n").unwrap_or_default();
  let warehouse = Warehouse::from(warehouse);
  let moves: Moves = moves
    .lines()
    .flat_map(|line| {
      line
        .chars()
        .map(|ch| match ch {
          '>' => Warehouse::RIGHT,
          'v' => Warehouse::DOWN,
          '<' => Warehouse::LEFT,
          '^' => Warehouse::UP,
          _ => unreachable!(),
        })
        .collect::<Moves>()
    })
    .collect();
  (moves, warehouse)
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
    assert_eq!("10092", process(input)?);
    Ok(())
  }
}
