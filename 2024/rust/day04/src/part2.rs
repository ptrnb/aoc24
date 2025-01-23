struct Grid {
  bytes: Vec<Vec<u8>>,
  rows: usize,
  cols: usize,
}

impl From<&str> for Grid {
  fn from(input: &str) -> Self {
    let bytes: Vec<Vec<u8>> = input.lines().map(|row| row.bytes().collect()).collect();
    let (rows, cols) = (bytes.len(), bytes.first().map_or(0, |row| row.len()));
    Self { bytes, rows, cols }
  }
}

impl Grid {
  fn get(&self, row: isize, col: isize) -> u8 {
    *self
      .bytes
      .get(row as usize)
      .and_then(|row| row.get(col as usize))
      .unwrap_or(&b'.')
  }

  fn crossmas_count(&self, row: usize, col: usize) -> bool {
    /*
     * Return true when the letters "MAS" are in an X shape
     *
     * . . . . . . .    . . . . . . .    . . . . . . .
     * . . M . S . .    . . M . . . .    . . . . S . .
     * . . . A . . .    . . . A . . .    . . . A . . .
     * . . M . S . .    . . . . S . .    . . M . . . .
     * . . . . . . .    . . . . . . .    . . . . . . .
     *    cross          back_slash      forward_slash
     */
    // row and col represent the coords of each letter "A"
    // convert to isize to satisfy signature to Grid::get() method
    let (row, col) = (row as isize, col as isize);
    // get the chars on the diagonals around the letter "A"
    let back_slash_letters = [self.get(row - 1, col - 1), self.get(row + 1, col + 1)];
    let forward_slash_letters = [self.get(row - 1, col + 1), self.get(row + 1, col - 1)];
    // Check both diagonals - both must only contain one each of "M" and "S"
    [back_slash_letters, forward_slash_letters]
      .iter()
      .all(|letters| letters == b"MS" || letters == b"SM")
  }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let grid = Grid::from(input);
  Ok(
    (0..grid.rows)
      .flat_map(|row| (0..grid.cols).map(move |col| (row, col)))
      .filter(|&(row, col)| grid.bytes[row][col] == b'A')
      .filter(|&(row, col)| grid.crossmas_count(row, col))
      .count()
      .to_string(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!("9", process(input)?);
    Ok(())
  }
}
