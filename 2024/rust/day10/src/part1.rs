use std::collections::HashSet;

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
  fn get(&self, (x, y): (i32, i32)) -> Option<u32> {
    self.terrain.get(x as usize)?.get(y as usize).copied()
  }

  fn count_trails(&self, (x, y): (i32, i32)) -> u32 {
    let mut stack = vec![];
    let mut visited = HashSet::new();
    let deltas = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    // Coordinates for start of trail
    stack.push((x, y, 0));

    while let Some((x, y, height)) = stack.pop() {
      if visited.insert((x, y, height)) {
        deltas.iter().for_each(|(delta_x, delta_y)| {
          if let Some(next_step) = self.get((x + delta_x, y + delta_y)) {
            if next_step == height + 1 {
              stack.push((x + delta_x, y + delta_y, next_step));
            }
          }
        })
      } else {
        continue;
      };
    }
    visited.iter().filter(|(_, _, height)| *height == 9).count() as u32
  }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let terrain = Map::from(input);
  Ok(
    terrain
      .trail_heads
      .iter()
      .map(|(x, y)| terrain.count_trails((*x, *y)))
      .sum::<u32>()
      .to_string(),
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
    assert_eq!("36", process(input)?);
    Ok(())
  }
}
