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

  fn xmas_count(&self, row: usize, col: usize) -> usize {
    [
      (0, 1),   // down
      (0, -1),  // up
      (1, 0),   // right
      (-1, 0),  // left
      (1, 1),   // right and down
      (1, -1),  // right and up
      (-1, 1),  // left and down
      (-1, -1), // left and up
    ]
    .iter()
    .filter(|(off_x, off_y)| {
      (1..4).all(|i| {
        let new_row = row as isize + (off_x * i);
        let new_col = col as isize + (off_y * i);
        self.get(new_row, new_col) == b"XMAS"[i as usize]
      })
    })
    .count()
  }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let grid = Grid::from(input);
  let mut counter = 0;
  (0..grid.rows)
    .flat_map(|row| (0..grid.cols).map(move |col| (row, col)))
    .filter(|&(row, col)| grid.bytes[row][col] == b'X')
    .for_each(|(row, col)| counter += grid.xmas_count(row, col));
  Ok(counter.to_string())
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
    assert_eq!("18", process(input)?);
    Ok(())
  }
}
