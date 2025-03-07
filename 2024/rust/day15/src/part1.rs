use glam::I64Vec2;
use std::{collections::HashMap, fmt::Display};

type Moves = Vec<I64Vec2>;

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

#[derive(Debug, PartialEq, Eq)]
enum Item {
  Box,
  Wall,
}

impl Display for Item {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Item::Box => write!(f, "O"),
      Item::Wall => write!(f, "#"),
    }
  }
}

type Grid = HashMap<I64Vec2, Item>;

struct Warehouse {
  height: i64,
  width: i64,
  floorplan: Grid,
  robot: I64Vec2,
}

impl From<&str> for Warehouse {
  fn from(input: &str) -> Self {
    let mut floorplan = Grid::new();
    let mut robot = I64Vec2::ZERO;
    let mut rows = 0;
    let mut cols = 0;

    for (row, line) in input.lines().enumerate() {
      for (col, chr) in line.chars().enumerate() {
        let pos = I64Vec2::new(row as i64, col as i64);

        if chr == '#' {
          floorplan.insert(pos, Item::Wall);
        } else if chr == 'O' {
          floorplan.insert(pos, Item::Box);
        } else if chr == '@' {
          robot = pos;
        }

        rows = if row > rows { row } else { rows };
        cols = if col > cols { col } else { cols };
      }
    }

    Self {
      height: rows as i64,
      width: cols as i64,
      floorplan,
      robot,
    }
  }
}

impl Display for Warehouse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Warehouse height {}", self.height + 1)?;
    writeln!(f, "Warehouse width {}", self.width + 1)?;

    for row in 0..=self.height {
      for col in 0..=self.width {
        let pos = I64Vec2::new(row, col);
        match self.floorplan.get(&pos) {
          None => write!(f, ".")?,
          Some(item) => write!(f, "{item}")?,
        }
      }
      writeln!(f)?;
    }
    writeln!(f)?;
    Ok(())
  }
}

impl Warehouse {
  fn try_move(&mut self, pos: &I64Vec2, direction: &I64Vec2) -> bool {
    match self.floorplan.get(pos) {
      None => true,
      Some(Item::Wall) => false,
      Some(Item::Box) => {
        let next_position = pos + direction;
        if self.try_move(&next_position, direction) {
          self.floorplan.remove(pos);
          self.floorplan.insert(next_position, Item::Box);
          true
        } else {
          false
        }
      }
    }
  }

  fn box_score(&self) -> i64 {
    self
      .floorplan
      .iter()
      .filter_map(|(pos, item)| {
        if *item == Item::Box {
          Some(pos.x * 100 + pos.y)
        } else {
          None
        }
      })
      .sum()
  }
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
