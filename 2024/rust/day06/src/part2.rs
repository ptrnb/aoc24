mod maze;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let mut labyrinth = maze::Labyrinth::from(input);
  let maze_path = labyrinth.walk();

  let count = maze_path
    .iter()
    .filter(|&&possible_obstacle| labyrinth.looping(possible_obstacle))
    .count() as u32;
  Ok(count.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!("6", process(input)?);
    Ok(())
  }
}
