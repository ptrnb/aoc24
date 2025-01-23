use std::{collections::HashSet, ops::Add};

type Grid = Vec<Vec<u8>>;

#[derive(Debug)]
pub struct Labyrinth {
  pub grid: Grid,
  pub guard: Guard,
}

impl From<&str> for Labyrinth {
  fn from(input: &str) -> Self {
    let parsed_grid: Grid = input.lines().map(|line| line.bytes().collect()).collect();
    let home = parsed_grid
      .iter()
      .enumerate()
      .find_map(|(x, row)| {
        row
          .iter()
          .position(|&b| b == b'^')
          .map(|y| Point(x as i32, y as i32))
      })
      .unwrap_or_default();
    Self {
      grid: parsed_grid,
      guard: Guard {
        position: home,
        direction: Direction::Up,
      },
    }
  }
}

impl Labyrinth {
  pub fn walk(&mut self) -> HashSet<Point> {
    let mut path: HashSet<Point> = HashSet::new();
    loop {
      path.insert(self.guard.position);
      let next = self.guard.position + self.guard.direction.offset();
      match self.get(next) {
        Some(b'#') => self.guard.direction = self.guard.direction.turn(),
        Some(_) => self.guard.position = next,
        None => break,
      }
    }
    path
  }
  fn get(&self, Point(x, y): Point) -> Option<u8> {
    self.grid.get(x as usize)?.get(y as usize).copied()
    // outer vec ^^^^^^^^^^^ inner ^ ^^^^^^^^^
  }
}

#[derive(Debug)]
pub struct Guard {
  pub position: Point,
  pub direction: Direction,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Point(i32, i32);

impl Add<Offset> for Point {
  type Output = Self;
  fn add(self, Offset(dx, dy): Offset) -> Self::Output {
    Self(self.0 + dx, self.1 + dy)
  }
}

struct Offset(i32, i32);

#[derive(Debug)]
pub enum Direction {
  Up,
  Down,
  Right,
  Left,
}

impl Direction {
  fn offset(&self) -> Offset {
    match self {
      Direction::Up => Offset(-1, 0),
      Direction::Down => Offset(1, 0),
      Direction::Right => Offset(0, 1),
      Direction::Left => Offset(0, -1),
    }
  }

  fn turn(&self) -> Self {
    match self {
      Direction::Up => Direction::Right,
      Direction::Down => Direction::Left,
      Direction::Right => Direction::Down,
      Direction::Left => Direction::Up,
    }
  }
}
