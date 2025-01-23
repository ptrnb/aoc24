pub fn process(input: &str) -> miette::Result<String> {
  let mut safe_reports = 0;

  for line in input.lines() {
    let report: Vec<i32> = line
      .split_whitespace()
      .filter_map(|n| n.parse().ok()) // convert to i32
      .collect();

    if is_safe(&report) || is_safe_after_dampen(&report) {
      safe_reports += 1;
    }
  }
  Ok(safe_reports.to_string())
}

// Helper functions
fn is_safe(report: &[i32]) -> bool {
  let is_increasing = report
    .windows(2)
    .all(|level| (level[0] <= level[1]) && (1..=3).contains(&(level[1] - level[0])));
  let is_decreasing = report
    .windows(2)
    .all(|level| (level[0] >= level[1]) && (1..=3).contains(&(level[0] - level[1])));

  is_increasing || is_decreasing
}

fn is_safe_after_dampen(report: &[i32]) -> bool {
  let mut new_report = report.to_vec();
  for (i, _) in new_report.clone().iter().enumerate() {
    let removed = new_report.remove(i);
    if is_safe(&new_report) {
      return true;
    };
    new_report.insert(i, removed);
  }
  false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
    assert_eq!(String::from("4"), process(input)?);
    Ok(())
  }
}
