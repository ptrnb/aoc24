use std::collections::{HashMap, HashSet};

use itertools::Itertools;

// For part 2 where we count corners the order of these points is important.
// We need to make sure we order these points so `circular_tuple_windows`
// does not give us an east/west or north/south combination. We need to move
// around each plot position
//
// N,E -> E,S -> S,W -> W,N
//
const COMPASS: [(i32, i32); 4] = [
  (0, -1), // North
  (1, 0),  // East
  (0, 1),  // South
  (-1, 0), // West
];

#[derive(Debug, Clone)]
struct Garden {
  plots: HashMap<(i32, i32), char>,
  cached_map: HashMap<(i32, i32), char>,
}

impl From<&str> for Garden {
  fn from(input: &str) -> Self {
    let mut plots: HashMap<(i32, i32), char> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
      for (col, vegetable) in line.chars().enumerate() {
        plots.insert((row as i32, col as i32), vegetable);
      }
    }
    let cached_map = plots.clone();

    // We mutate Garden.plots while calculating area of each vege plot
    //
    // We borrow Garden.garden_map for calculating the number of
    // sides (corners) that each vege plot has.
    Self { plots, cached_map }
  }
}

impl Garden {
  fn get_vegetable(&self, location: &(i32, i32)) -> Option<char> {
    self.plots.get(location).copied()
  }

  fn remove(&mut self, location: &(i32, i32)) {
    self.plots.remove(location);
  }

  fn find_plots(&mut self, position: (i32, i32)) -> (usize, usize) {
    let mut plot_positions = vec![position];
    let mut seen = HashSet::new();
    let vegetable = self.get_vegetable(&position).unwrap_or('.');

    // Remove the positions we have already seen from the plot
    self.remove(&position);

    // Check neighbours for matching vegetables
    while let Some(current_pos) = plot_positions.pop() {
      if seen.insert(current_pos) {
        // We have not seen this location before
        for direction in COMPASS {
          let neighbour = (current_pos.0 + direction.0, current_pos.1 + direction.1);
          if vegetable == self.get_vegetable(&neighbour).unwrap_or('.') {
            self.remove(&neighbour);
            plot_positions.push(neighbour);
          }
        }
      }
    }

    let area = seen.len();

    let sides = seen
      .iter()
      .map(|plot_spot| self.count_corners(&plot_spot, &vegetable))
      .sum();

    (area, sides)
  }

  fn count_corners(&self, position: &(i32, i32), vegetable: &char) -> usize {
    let mut side_count = 0;

    for ((x1, y1), (x2, y2)) in COMPASS.iter().circular_tuple_windows() {
      let test_a = self
        .cached_map
        .get(&(x1 + position.0, y1 + position.1))
        .is_some_and(|v| v == vegetable);

      let test_b = self
        .cached_map
        .get(&(x2 + position.0, y2 + position.1))
        .is_some_and(|v| v == vegetable);

      if test_a
        && test_b
        && self
          .cached_map
          .get(&(x1 + x2 + position.0, y1 + y2 + position.1))
          .is_some_and(|v| v != vegetable)
      {
        // interior corner
        side_count += 1;
      } else if !test_a && !test_b {
        // exterior corner
        side_count += 1;
      }
    }
    side_count
  }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
  let mut garden = Garden::from(input);
  let mut total = 0;
  while let Some(position) = garden.plots.keys().copied().next() {
    let (area, sides) = garden.find_plots(position);
    total += area * sides;
  }

  Ok(total)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    assert_eq!(1206, process(input)?);
    Ok(())
  }
}
