use std::{collections::HashSet, ops::Add};

type Grid = Vec<Vec<u8>>;

#[derive(Debug)]
pub struct Labyrinth {
  pub grid: Grid,
  pub guard: Guard,
  pub origin: Guard,
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
      origin: Guard {
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

  fn set(&mut self, Point(x, y): Point, value: u8) {
    if let Some(cell) = self
      .grid
      .get_mut(x as usize)
      .and_then(|row| row.get_mut(y as usize))
    {
      *cell = value;
    };
  }

  pub fn looping(&mut self, obstacle: Point) -> bool {
    let mut loop_path = HashSet::new();
    self.guard = self.origin;

    // Set a new obstacle at the current position (obstacle) on the maze path
    self.set(obstacle, b'O');

    let creates_loop = loop {
      // Re-walk the maze path.
      // Check if the new obstacle causes us to revisit
      //   any point on the maze path with the same direction
      // If so, we have confirmed the new obstacle
      //   has created an infinite loop in the labyrinth!
      if !loop_path.insert((self.guard.position, self.guard.direction)) {
        break true;
      }
      // Get the location of the next point
      let next = self.guard.position + self.guard.direction.offset();
      match self.get(next) {
        // Check if the next point blocks us and then turn
        Some(b'#' | b'O') => self.guard.direction = self.guard.direction.turn(),
        // Otherwise move to the next point
        Some(_) => self.guard.position = next,
        // Or we have reached the boundary of the maze and need to stop searching
        None => break false,
      }
    };
    self.set(obstacle, b'.');
    creates_loop
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
