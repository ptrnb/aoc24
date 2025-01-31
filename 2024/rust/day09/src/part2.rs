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

  fn defragment_by_block(&mut self) -> Self {
    let mut defragged = VecDeque::new();
    // Walk the diskmap forwards
    while let Some(block) = self.disk.pop_front() {
      match block {
        Block::File { .. } => defragged.push_back(block),
        Block::Free {
          size: mut free_size,
        } => {
          // Walk the diskmap backwards using indices
          (0..self.disk.len()).rev().into_iter().for_each(|i| {
            if let Block::File {
              size: file_size, ..
            } = self.disk[i]
            {
              if file_size <= free_size {
                defragged.push_back(self.disk[i]);
                self.disk.remove(i);
                self.disk.insert(i, Block::Free { size: file_size });
                free_size -= file_size;
              };
            }
          });
          if free_size > 0 {
            defragged.push_back(Block::Free { size: free_size });
          };
        }
      }
    }
    Self { disk: defragged }
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
      .defragment_by_block()
      .decompress()
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
