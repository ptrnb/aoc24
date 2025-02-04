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
  // Alternative types used for match branches below
  type L = [u64; 1];
  type R = [u64; 2];

  let magic_stones = MagicStones::from(input);

  let mut all_blinks = std::iter::successors(Some(magic_stones.stones), |stones| {
    let next_stones: Vec<u64> = stones
      .iter()
      // Use enum Either to handle alternative types A and B
      // returned by match branches
      .flat_map(|stone| match stone {
        0 => Either::<L, R>::Left([1]).into_iter(),

        n if (n.ilog10() + 1) % 2 == 0 => {
          // Use math to "split" a number with an even number of digits
          // e.g. 123456 -> 123 & 456
          //      100053 -> 100 & 53
          let number_len = &n.ilog10() + 1;
          let divisor = 10u64.pow(number_len / 2);
          let (left, right) = Euclid::div_rem_euclid(n, &divisor);
          Either::<L, R>::Right([left, right]).into_iter()
        }

        n => Either::<L, R>::Left([n * 2024]).into_iter(),
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
    let input = "125 17";
    assert_eq!("55312", process(input)?);
    Ok(())
  }
}
