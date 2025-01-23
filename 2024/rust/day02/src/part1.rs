pub fn process(input: &str) -> miette::Result<String> {
  let mut safe_reports = 0;

  for line in input.lines() {
    let report: Vec<i32> = line
      .split_whitespace()
      .filter_map(|n| n.parse().ok()) // convert to i32
      .collect();

    if is_safe(&report) {
      safe_reports += 1;
    }
  }
  Ok(safe_reports.to_string())
}

fn is_safe(report: &[i32]) -> bool {
  let is_increasing = report
    .windows(2)
    .all(|level| (level[0] <= level[1]) && (1..=3).contains(&(level[1] - level[0])));
  let is_decreasing = report
    .windows(2)
    .all(|level| (level[0] >= level[1]) && (1..=3).contains(&(level[0] - level[1])));

  is_increasing || is_decreasing
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!("2", process(input)?);
    Ok(())
  }
}
