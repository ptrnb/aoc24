use miette::miette;

use crate::parsers::parse;
use crate::types::{Robot, GRID_SIZE};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let (_input, mut robots) = parse(input).map_err(|e| miette!("Error parsing {}", e))?;
  for _ in 0..100 {
    for robot in robots.iter_mut() {
      robot.position = (robot.position + robot.velocity).rem_euclid(GRID_SIZE);
    }
  }
  // debug_robots(&robots);

  let divider = GRID_SIZE / 2;

  let quadrants = [
    (0..divider.x, 0..divider.y),
    ((divider.x + 1)..GRID_SIZE.x, 0..divider.y),
    (0..divider.x, (divider.y + 1)..GRID_SIZE.y),
    ((divider.x + 1)..GRID_SIZE.x, (divider.y + 1)..GRID_SIZE.y),
  ];

  let result: usize = quadrants
    .iter()
    .map(|(xs, ys)| {
      robots
        .iter()
        .filter(|Robot { position, .. }| xs.contains(&position.x) && ys.contains(&position.y))
        .count()
    })
    .product();

  Ok(result.to_string())
}

/* fn debug_robots(robots: &[Robot]) {
  println!("");
  for row_pos in 0..GRID_SIZE.y {
    for col_pos in 0..GRID_SIZE.x {
      match robots
        .iter()
        // Destructure the robot position
        .filter(|Robot { position, .. }| position.x == col_pos && position.y == row_pos)
        .count()
      {
        0 => print!("."),
        n => print!("{}", n),
      }
    }
    println!("");
  }
  println!("");
}
*/
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    assert_eq!("12", process(input)?);
    Ok(())
  }
}
