use num::traits::Euclid;
use std::collections::HashMap;

#[derive(Debug)]
struct MagicStones {
  counter: HashMap<u64, u64>,
}

impl From<&str> for MagicStones {
  fn from(input: &str) -> Self {
    let stones: Vec<u64> = input
      .split_whitespace()
      .filter_map(|n| n.parse().ok())
      .collect();

    let mut counter: HashMap<u64, u64> = HashMap::default();

    for stone in stones {
      counter
        .entry(stone)
        .and_modify(|val| *val += 1)
        .or_insert(1);
    }

    Self { counter }
  }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let mut stones = MagicStones::from(input);

  for _ in 0..75 {
    let mut new_counter: HashMap<u64, u64> = HashMap::default();

    for (num, count) in stones.counter.into_iter() {
      match num {
        0 => {
          new_counter
            .entry(1)
            .and_modify(|v| *v += count)
            .or_insert(count);
        }

        n if (n.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
          let divisor = 10u64.pow((&n.ilog10() + 1) / 2);
          let (left, right) = Euclid::div_rem_euclid(&n, &divisor);
          new_counter
            .entry(left)
            .and_modify(|v| *v += count)
            .or_insert(count);
          new_counter
            .entry(right)
            .and_modify(|v| *v += count)
            .or_insert(count);
        }

        n => {
          new_counter
            .entry(n * 2024)
            .and_modify(|v| *v += count)
            .or_insert(count);
        }
      }
    }
    stones.counter = new_counter;
  }
  Ok(stones.counter.values().sum::<u64>().to_string())
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
