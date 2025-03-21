use glam::I64Vec2;
use pathfinding::prelude::astar_bag_collect;
use std::{collections::HashSet, fmt::Display};

const EAST: I64Vec2 = I64Vec2::Y;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let maze = Maze::from(input);
  let start_location = MazeTracker {
    location: maze.start,
    direction: EAST,
  };

  let Some((paths, _cost)) = maze.all_shortest_paths(&start_location) else {
    panic!("No path found for this maze")
  };

  let mut steps = HashSet::new();
  for path in paths {
    steps.extend(path.iter().map(|a_step| a_step.location));
  }
  Ok(steps.len().to_string())
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Hash)]
struct MazeTracker {
  location: I64Vec2,
  direction: I64Vec2,
}

#[derive(Debug, Eq, PartialEq)]
struct Maze {
  walls: HashSet<I64Vec2>,
  rows: i64,
  cols: i64,
  start: I64Vec2,
  end: I64Vec2,
}

impl From<&str> for Maze {
  fn from(input: &str) -> Self {
    let mut walls = HashSet::new();
    let mut start = I64Vec2::ZERO;
    let mut end = I64Vec2::ZERO;
    let mut rows: i64 = 0;
    let mut cols: i64 = 0;
    for (row, line) in input.lines().enumerate() {
      for (col, ch) in line.chars().enumerate() {
        rows = if row as i64 > rows { row as i64 } else { rows };
        cols = if col as i64 > cols { col as i64 } else { cols };

        let location = I64Vec2::new(row as i64, col as i64);

        if 'S' == ch {
          start = location;
        } else if 'E' == ch {
          end = location;
        } else if '#' == ch {
          walls.insert(location);
        }
      }
    }
    Self {
      walls,
      start,
      end,
      rows,
      cols,
    }
  }
}

impl Display for Maze {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for row in 0..=self.rows {
      for col in 0..=self.cols {
        let pos = I64Vec2::new(row, col);
        if self.walls.contains(&pos) {
          write!(f, "#")?;
        } else if pos == self.start {
          write!(f, "S")?;
        } else if pos == self.end {
          write!(f, "E")?;
        } else {
          write!(f, ".")?;
        }
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

impl Maze {
  fn all_shortest_paths(
    &self,
    start_location: &MazeTracker,
  ) -> Option<(Vec<Vec<MazeTracker>>, usize)> {
    astar_bag_collect(
      start_location,
      |step| {
        // Must return type with IntoIter
        let mut result = Vec::<(MazeTracker, usize)>::new();
        let next_step = step.location + step.direction;
        if !self.walls.contains(&next_step) {
          result.push((
            MazeTracker {
              location: next_step,
              direction: step.direction,
            },
            // Score is 1 point for the move
            1,
          ));
        }
        let left_turn = I64Vec2::new(-step.direction.y, step.direction.x);
        let next_step = step.location + left_turn;
        if !self.walls.contains(&next_step) {
          result.push((
            MazeTracker {
              location: next_step,
              direction: left_turn,
            },
            // Score is 1 point for the move + 1000 points for turning
            1001,
          ));
        }
        let right_turn = I64Vec2::new(step.direction.y, -step.direction.x);
        let next_step = step.location + right_turn;
        if !self.walls.contains(&next_step) {
          result.push((
            MazeTracker {
              location: next_step,
              direction: right_turn,
            },
            // Score is 1 point for the move + 1000 points for turning
            1001,
          ));
        }
        result
      },
      |step| (step.location.x.abs_diff(self.end.x) + step.location.y.abs_diff(self.end.y)) as usize,
      |step| step.location == self.end,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    assert_eq!("45", process(input)?);
    Ok(())
  }
}
