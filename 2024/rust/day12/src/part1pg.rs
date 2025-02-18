use petgraph::{algo::condensation, prelude::*, visit::IntoNodeReferences};

use std::collections::HashMap;

const COMPASS: [(i32, i32); 4] = [
  (0, -1), // North
  (0, 1),  // South
  (1, 0),  // East
  (-1, 0), // West
];

#[derive(Debug)]
struct Garden {
  plots: HashMap<(i32, i32), char>,
}

impl From<&str> for Garden {
  fn from(input: &str) -> Self {
    let plots = input
      .lines()
      .enumerate()
      .flat_map(|(row, line)| {
        line
          .chars()
          .enumerate()
          .map(move |(col, chr)| ((row as i32, col as i32), chr))
      })
      .collect::<HashMap<(i32, i32), char>>();

    Self { plots }
  }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
  let garden = Garden::from(input);

  let mut graph_of_plots: UnGraphMap<(i32, i32), ()> = UnGraphMap::new();

  for ((row, col), veg1) in garden.plots.iter() {
    let node = graph_of_plots.add_node((*row, *col));
    for dir in COMPASS.iter() {
      let new_node = (row + dir.0, col + dir.1);
      if garden.plots.get(&new_node).is_some_and(|veg2| veg1 == veg2) {
        graph_of_plots.add_edge(node, new_node, ());
      };
    }
  }

  let new_graph_of_plots = condensation(graph_of_plots.clone().into_graph::<NodeIndex>(), false);

  let result = new_graph_of_plots
    .node_references()
    .map(|(_node_index, node_list)| {
      let area = node_list.len();
      let perimeter = node_list
        .iter()
        .map(|n| 4 - graph_of_plots.neighbors(*n).count())
        .sum::<usize>();
      area * perimeter
    })
    .sum::<usize>();

  Ok(result)
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
