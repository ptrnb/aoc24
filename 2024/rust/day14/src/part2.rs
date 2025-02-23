use std::collections::HashSet;

use glam::IVec2;
use miette::miette;

use crate::parsers::parse;
use crate::types::{Robot, GRID_SIZE};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let (_input, mut robots) = parse(input).map_err(|e| miette!("Error parsing {}", e))?;

  // Significant patterns show up at the following intervals
  // h: step 86, 189 ... horizontal patterns appear every 103 steps
  // v: step 28, 129, ... vertical patterns appear every 101 steps

  let mut result = 0;
  for sec in 0..10_000 {
    // Track the significant patterns
    //  - this gets us straight to the answer: 7502
    if sec % 103 == 86 && sec % 101 == 28 {
      println!("After {} seconds", sec);
      display_grid(&robots);
      result = sec;
      break;
    };
    for robot in robots.iter_mut() {
      robot.position = (robot.position + robot.velocity).rem_euclid(GRID_SIZE);
    }
  }

  Ok(result.to_string())
}

fn display_grid(robots: &Vec<Robot>) {
  let grid = robots
    .iter()
    .map(|robot| robot.position)
    .collect::<HashSet<_>>();

  for y in 0..GRID_SIZE.y {
    for x in 0..GRID_SIZE.x {
      print!(
        "{}",
        if grid.contains(&IVec2::new(x, y)) {
          "#"
        } else {
          "."
        }
      );
    }
    println!();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    todo!("haven't built test yet");
    let input = "";
    assert_eq!("", process(input)?);
    Ok(())
  }
}
