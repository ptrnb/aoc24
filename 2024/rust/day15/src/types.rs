use glam::I64Vec2;
use std::{
  collections::{HashMap, HashSet},
  fmt::Display,
};

// Part 1

pub type Moves = Vec<I64Vec2>;

#[derive(Debug, PartialEq, Eq)]
pub enum Item {
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

pub struct Warehouse {
  pub height: i64,
  pub width: i64,
  pub floorplan: Grid,
  pub robot: I64Vec2,
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

        rows = if pos.x > rows { pos.x } else { rows };
        cols = if pos.y > cols { pos.y } else { cols };
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
        if pos == self.robot {
          write!(f, "@")?;
          continue;
        };
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
  pub const RIGHT: I64Vec2 = I64Vec2::Y;
  pub const DOWN: I64Vec2 = I64Vec2::X;
  pub const LEFT: I64Vec2 = I64Vec2::NEG_Y;
  pub const UP: I64Vec2 = I64Vec2::NEG_X;

  pub fn try_move(&mut self, pos: &I64Vec2, direction: &I64Vec2) -> bool {
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

  pub fn box_score(&self) -> i64 {
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

// Part 2

#[derive(Debug, PartialEq, Eq)]
pub enum WideItem {
  BoxLeft,
  BoxRight,
  Wall,
}

impl Display for WideItem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      WideItem::BoxLeft => write!(f, "["),
      WideItem::BoxRight => write!(f, "]"),
      WideItem::Wall => write!(f, "#"),
    }
  }
}

type WideGrid = HashMap<I64Vec2, WideItem>;

pub struct WideWarehouse {
  pub height: i64,
  pub width: i64,
  pub floorplan: WideGrid,
  pub robot: I64Vec2,
}

impl From<&str> for WideWarehouse {
  fn from(input: &str) -> Self {
    let mut floorplan = WideGrid::new();
    let mut robot = I64Vec2::ZERO;
    let mut rows = 0;
    let mut cols = 0;

    let expand_warehouse: Vec<Vec<char>> = input
      .lines()
      .map(|line| {
        line
          .chars()
          .flat_map(|c| match c {
            '#' => vec!['#', '#'],
            'O' => vec!['[', ']'],
            '@' => vec!['@', '.'],
            '.' => vec!['.', '.'],
            _ => unreachable!(),
          })
          .collect()
      })
      .collect();

    for (row, line) in expand_warehouse.iter().enumerate() {
      for (col, chr) in line.iter().enumerate() {
        let pos = I64Vec2::new(row as i64, col as i64);

        if *chr == '#' {
          floorplan.insert(pos, WideItem::Wall);
        } else if *chr == '[' {
          floorplan.insert(pos, WideItem::BoxLeft);
        } else if *chr == ']' {
          floorplan.insert(pos, WideItem::BoxRight);
        } else if *chr == '@' {
          robot = pos;
        }

        rows = if pos.x > rows { pos.x } else { rows };
        cols = if pos.y > cols { pos.y } else { cols };
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

impl Display for WideWarehouse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Warehouse height {}", self.height + 1)?;
    writeln!(f, "Warehouse width {}", self.width + 1)?;

    for row in 0..=self.height {
      for col in 0..=self.width {
        let pos = I64Vec2::new(row, col);
        if pos == self.robot {
          write!(f, "@")?;
          continue;
        };
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

impl WideWarehouse {
  pub const RIGHT: I64Vec2 = I64Vec2::Y;
  pub const DOWN: I64Vec2 = I64Vec2::X;
  pub const LEFT: I64Vec2 = I64Vec2::NEG_Y;
  pub const UP: I64Vec2 = I64Vec2::NEG_X;

  pub fn try_big_move(&mut self, pos: I64Vec2, direction: I64Vec2) -> bool {
    match direction.x {
      0 => {
        // Moving horizontally
        match self.floorplan.get(&pos) {
          None => true,
          Some(WideItem::Wall) => false,
          Some(WideItem::BoxRight) => {
            let next_position = pos + direction;
            if self.try_big_move(next_position, direction) {
              self.floorplan.remove(&pos);
              self.floorplan.insert(next_position, WideItem::BoxRight);
              true
            } else {
              false
            }
          }
          Some(WideItem::BoxLeft) => {
            let next_position = pos + direction;
            if self.try_big_move(next_position, direction) {
              self.floorplan.remove(&pos);
              self.floorplan.insert(next_position, WideItem::BoxLeft);
              true
            } else {
              false
            }
          }
        }
      }
      -1 | 1 => {
        /*
         * Moving upwards or downwards
         *
         * call function to return a bool, list of coords to move in direction
         *
         * we have the current pos and dir as inputs
         *
         * function must travel up or down all boxes in the stack
         *   and assess if there is room at the end to move all of them
         *
         * function needs to know if it assessing a BoxLeft or BoxRight
         *   as it moves up and down the stack of boxes.
         *
         * If we hit a wall along the way - abandon the move and return
         *   false with an empty movelist
         */
        let (allowed, mut movelist) = match self.floorplan.get(&pos) {
          None => return true,
          Some(item) => match item {
            WideItem::Wall => return false,
            WideItem::BoxLeft => self.find_all_boxes_that_can_move(pos, direction),
            WideItem::BoxRight => {
              let left_side = pos + Self::LEFT;
              self.find_all_boxes_that_can_move(left_side, direction)
            }
          },
        };
        if allowed {
          while !movelist.is_empty() {
            let moveset = movelist.drain(..).collect::<HashSet<_>>();
            for left_side in moveset {
              let right_side = left_side + Self::RIGHT;
              let next_box_left = left_side + direction;
              let next_box_right = left_side + Self::RIGHT + direction;
              if self.floorplan.contains_key(&next_box_left)
                || self.floorplan.contains_key(&next_box_right)
              {
                movelist.push(left_side);
              } else {
                assert_eq!(self.floorplan.get(&left_side), Some(&WideItem::BoxLeft));
                self.floorplan.remove(&left_side);
                self.floorplan.remove(&right_side);
                self.floorplan.insert(next_box_left, WideItem::BoxLeft);
                self.floorplan.insert(next_box_right, WideItem::BoxRight);
              }
            }
          }
        }
        allowed
      }
      _ => unreachable!(
        "We got an instruction to move in direction that wasn't up, down, left or right"
      ),
    }
  }

  fn find_all_boxes_that_can_move(
    &self,
    mut side: I64Vec2,
    direction: I64Vec2,
  ) -> (bool, Vec<I64Vec2>) {
    // Return (allowed, Vec<BoxLeft coords>)
    if Some(&WideItem::BoxRight) == self.floorplan.get(&side) {
      // Reposition ourselves on a BoxLeft, if needed
      side = side + Self::LEFT;
    }
    let next_position_left = side + direction;
    let next_position_right = side + direction + Self::RIGHT;
    // Peek at what is sitting in the next position along
    // from our current position (side)
    match (
      self.floorplan.get(&next_position_left),
      self.floorplan.get(&next_position_right),
    ) {
      // Empty space
      // We can move both sides of the box in our current position
      (None, None) => (true, vec![side]),
      // Oops, there is a wall
      // We can't move
      (_, Some(WideItem::Wall)) | (Some(WideItem::Wall), _) => (false, vec![]),
      // Found another box
      // Check whether we are looking at BoxLeft or BoxRight
      // and then peek at one more step in the direction of travel
      (left, right) => {
        let mut movelist = vec![side];
        let (can_move, v) = match left {
          None => (true, vec![]),
          Some(WideItem::BoxRight) => {
            // We looked left, but found a BoxRight
            // So reposition one more step to the left and check again
            let one_more_step_left = next_position_left + Self::LEFT;
            self.find_all_boxes_that_can_move(one_more_step_left, direction)
          }
          Some(WideItem::BoxLeft) => {
            self.find_all_boxes_that_can_move(next_position_left, direction)
          }
          _ => unreachable!(),
        };
        if !can_move {
          return (false, v);
        }
        movelist.extend(v);
        let (can_move, v) = match right {
          None => (true, vec![]),
          Some(WideItem::BoxRight) => (true, vec![]),
          Some(WideItem::BoxLeft) => {
            // We looked right, but found a BoxLeft
            // So keep moving in the direction of travel and check again
            self.find_all_boxes_that_can_move(next_position_right, direction)
          }
          _ => unreachable!(),
        };
        if !can_move {
          return (false, v);
        }
        movelist.extend(v);
        (true, movelist)
      }
    }
  }

  pub fn box_score(&self) -> i64 {
    self
      .floorplan
      .iter()
      .filter_map(|(pos, item)| {
        if *item == WideItem::BoxLeft {
          Some(pos.x * 100 + pos.y)
        } else {
          None
        }
      })
      .sum()
  }
}
