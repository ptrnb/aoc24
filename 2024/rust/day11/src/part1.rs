use either::Either;
use num::traits::Euclid;

#[derive(Debug)]
struct MagicStones {
  stones: Vec<u64>,
}

impl From<&str> for MagicStones {
  fn from(input: &str) -> Self {
    Self {
      stones: input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect(),
    }
  }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let magic_stones = MagicStones::from(input);

  let mut all_blinks = std::iter::successors(Some(magic_stones.stones), |stones| {
    let next_stones: Vec<u64> = stones
      .iter()
      // Use crate Either to handle alternative types emitted by match branches
      .flat_map(|stone| match stone {
        0 => Either::<[u64; 1], [u64; 2]>::Left([1]).into_iter(),

        n if (n.ilog10() + 1) % 2 == 0 => {
          let divisor = 10u64.pow((&n.ilog10() + 1) / 2);
          let (left, right) = Euclid::div_rem_euclid(n, &divisor);
          let split_stone = Either::<[u64; 1], [u64; 2]>::Right([left, right]);
          split_stone.into_iter()
        }

        n => {
          let new_stone = Either::<[u64; 1], [u64; 2]>::Left([n * 2024]);
          new_stone.into_iter()
        }
      })
      .collect();
    Some(next_stones)
  });

  Ok(all_blinks.nth(25).unwrap().len().to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    // let input = "0 1 10 99 999";
    let input = "125 17";
    assert_eq!("55312", process(input)?);
    Ok(())
  }
}
