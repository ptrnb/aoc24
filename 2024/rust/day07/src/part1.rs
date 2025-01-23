type Equations = Vec<(u64, Vec<u64>)>;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let calibration_table: Equations = parse_input(input);

  Ok(
    calibration_table
      .iter()
      .filter(|(total, factors)| solve(total, factors))
      .map(|(test, _)| test)
      .sum::<u64>()
      .to_string(),
  )
}

fn solve(total: &u64, factors: &[u64]) -> bool {
  factors
    .iter()
    .skip(1)
    .fold(vec![factors[0]], |acc, &next_num| {
      acc
        .iter()
        .flat_map(|&previous_num| vec![previous_num + next_num, previous_num * next_num])
        .collect()
    })
    .contains(total)
}

fn parse_input(input: &str) -> Equations {
  input
    .lines()
    .filter_map(|line| {
      let (total, rest) = line.split_once(":")?;
      let total = total.trim().parse().ok()?;
      let factors = rest
        .split_whitespace()
        .filter_map(|i| i.parse().ok())
        .collect();
      Some((total, factors))
    })
    .collect::<Equations>()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    assert_eq!("3749", process(input)?);
    Ok(())
  }
}
