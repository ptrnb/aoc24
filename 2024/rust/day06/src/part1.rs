mod maze;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let mut labyrinth = maze::Labyrinth::from(input);
  let result = labyrinth.walk().len();
  Ok(result.to_string())
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
    assert_eq!("41", process(input)?);
    Ok(())
  }
}
