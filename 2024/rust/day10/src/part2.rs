const OFFSETS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Debug)]
struct Map {
  terrain: Vec<Vec<u32>>,
  trail_heads: Vec<(i32, i32)>,
}

impl From<&str> for Map {
  fn from(input: &str) -> Self {
    let terrain: Vec<Vec<u32>> = input
      .lines()
      .map(|line| line.chars().filter_map(|ch| ch.to_digit(10)).collect())
      .collect();

    // trails start at each 0 on the terrain
    let trail_heads = terrain
      .iter()
      .enumerate()
      .flat_map(|(x, row)| {
        row
          .iter()
          .enumerate()
          .filter_map(move |(y, &item)| (item == 0).then_some((x as i32, y as i32)))
      })
      .collect();

    Self {
      terrain,
      trail_heads,
    }
  }
}

impl Map {
  // Locate a point on the terrain by grid indexes
  // Return Some(value) or None if outside the bounds of the grid
  fn get(&self, (x, y): (i32, i32)) -> Option<u32> {
    self.terrain.get(x as usize)?.get(y as usize).copied()
  }

  fn count_trails(&self, (x, y): (i32, i32)) -> u32 {
    let mut stack = vec![];
    let mut visited = vec![];

    // Coordinates for start of trail
    stack.push((x, y, 0));

    while let Some((x, y, height)) = stack.pop() {
      visited.push((x, y, height));
      // Inspect up, down, left and right of current position
      OFFSETS.iter().for_each(|(offset_x, offset_y)| {
        if let Some(next_step) = self.get((x + offset_x, y + offset_y)) {
          if next_step == height + 1 {
            stack.push((x + offset_x, y + offset_y, next_step));
          }
        }
      })
    }
    // Count all the times we reach the end of each trail (9)
    visited.iter().filter(|(_, _, height)| *height == 9).count() as u32
  }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
  let terrain = Map::from(input);
  Ok(
    terrain
      .trail_heads
      .iter()
      .map(|(x, y)| terrain.count_trails((*x, *y)))
      .sum::<u32>(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    assert_eq!(81, process(input)?);
    Ok(())
  }
}
