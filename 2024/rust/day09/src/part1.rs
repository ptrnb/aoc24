use std::collections::VecDeque;

#[derive(Debug)]
struct DiskMap {
  disk: VecDeque<Block>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Block {
  File { id: usize, size: usize },
  Free { size: usize },
}

impl From<&str> for DiskMap {
  fn from(input: &str) -> Self {
    let disk = input
      .chars()
      .enumerate()
      //    init                  acc      item from enumerate
      .fold(VecDeque::new(), |mut blocks, (id, size)| {
        if let Some(size) = size.to_digit(10) {
          // File and Free alternate in the input
          // Use the idx from the enumerator to choose
          match id % 2 {
            // File
            0 => blocks.push_back(Block::File {
              id: id / 2,
              size: size as usize,
            }),
            // Free
            _ => blocks.push_back(Block::Free {
              size: size as usize,
            }),
          }
        }
        blocks
      });
    Self { disk }
  }
}

impl DiskMap {
  fn decompress(self) -> Self {
    Self {
      disk: self
        .disk
        .into_iter()
        .flat_map(|block| match block {
          Block::File { id, size } => vec![Block::File { id, size: 1 }; size],
          Block::Free { size } => vec![Block::Free { size: 1 }; size],
        })
        .collect(),
    }
  }

  fn defragment(&mut self) -> Self {
    let mut disk = VecDeque::new();
    while let Some(block) = self.disk.pop_front() {
      match block {
        Block::File { .. } => disk.push_back(block),
        Block::Free { .. } => {
          while let Some(block) = self.disk.pop_back() {
            if let Block::File { .. } = block {
              disk.push_back(block);
              break;
            }
          }
        }
      }
    }
    Self { disk }
  }

  fn checksum(&self) -> usize {
    self
      .disk
      .iter()
      .enumerate()
      .filter_map(|(idx, block)| match *block {
        Block::File { id, .. } => Some(id * idx),
        _ => None,
      })
      .sum()
  }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  Ok(
    DiskMap::from(input)
      .decompress()
      .defragment()
      .checksum()
      .to_string(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "2333133121414131402";
    assert_eq!("2858", process(input)?);
    Ok(())
  }
}
