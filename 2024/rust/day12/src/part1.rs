use std::collections::{HashMap, HashSet};

const COMPASS: [(i32, i32); 4] = [
  (0, -1), // North
  (1, 0),  // East
  (-1, 0), // West
  (0, 1),  // South
];

#[derive(Debug)]
struct Garden {
  plots: HashMap<(i32, i32), char>,
}

impl From<&str> for Garden {
  fn from(input: &str) -> Self {
    let mut plots: HashMap<(i32, i32), char> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
      for (col, vegetable) in line.chars().enumerate() {
        plots.insert((row as i32, col as i32), vegetable);
      }
    }
    /*
    let plots: HashMap<(i32, i32), char> = input
      .lines()
      .enumerate()
      .flat_map(|(row, line)| {
        line
          .chars()
          .enumerate()
          .map(move |(col, vegetable)| ((row as i32, col as i32), vegetable))
      })
      .collect();
    */

    Self { plots }
  }
}

impl Garden {
  fn get_vegetable(&self, location: &(i32, i32)) -> char {
    self.plots.get(location).copied().unwrap_or('.')
  }

  fn remove(&mut self, location: &(i32, i32)) {
    self.plots.remove(location);
  }

  fn find_plots(&mut self, position: (i32, i32)) -> (usize, usize) {
    let mut plot_positions = vec![position];
    let mut seen = HashSet::new();
    let vegetable = self.get_vegetable(&position);

    // Remove the positions we have already seen from the plot
    self.remove(&position);

    // Check neighbours for matching vegetables
    while let Some(current_pos) = plot_positions.pop() {
      if seen.insert(current_pos) {
        // We have not seen this location before
        for direction in COMPASS {
          let neighbour = (current_pos.0 + direction.0, current_pos.1 + direction.1);
          if vegetable == self.get_vegetable(&neighbour) {
            self.remove(&neighbour);
            plot_positions.push(neighbour);
          }
        }
      }
    }

    let area = seen.len();

    // Vegetable is on the perimeter if a neighbour doesn't match
    let mut perimeter = 0;
    for plot_spot in &seen {
      for direction in COMPASS {
        let neighbour = (plot_spot.0 + direction.0, plot_spot.1 + direction.1);
        if !seen.contains(&neighbour) {
          perimeter += 1;
        }
      }
    }
    (area, perimeter)
  }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
  let mut garden = Garden::from(input);
  let mut total = 0;
  while let Some(position) = garden.plots.keys().copied().next() {
    let (area, perimeter) = garden.find_plots(position);
    total += area * perimeter;
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
    assert_eq!(1930, process(input)?);
    Ok(())
  }
}
