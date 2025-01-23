use std::cmp::Ordering;

use nom::{
  self,
  bytes::complete::tag,
  character::complete::{self, line_ending},
  multi::{many1, separated_list1},
  sequence::{separated_pair, terminated},
  IResult,
};

type Rules = Vec<(u32, u32)>;
type Updates = Vec<Vec<u32>>;

#[derive(Debug)]
struct Manual {
  rules: Rules,
  updates: Updates,
}

impl From<&str> for Manual {
  fn from(input: &str) -> Self {
    let Ok((_input, (parsed_rules, parsed_updates))) = parse(input) else {
      panic!("An error occurred while parsing input and creating the Manual")
    };
    Self {
      rules: parsed_rules,
      updates: parsed_updates,
    }
  }
}

impl Manual {
  fn valid_update(&self, update: &[u32]) -> bool {
    self.rules.iter().all(|&(a, b)| {
      let before_page = update.iter().position(|&before| before == a);
      let after_page = update.iter().position(|&after| after == b);
      match (before_page, after_page) {
        (Some(before), Some(after)) => before < after,
        _ => true,
      }
    })
  }

  fn fix_order(&self, mut update: Vec<u32>) -> Vec<u32> {
    update.sort_by(|&first, &second| {
      if self.rules.contains(&(first, second)) {
        Ordering::Less
      } else if self.rules.contains(&(second, first)) {
        Ordering::Greater
      } else {
        Ordering::Equal
      }
    });
    update
  }
}

// Main parser - Combinator
fn parse(input: &str) -> IResult<&str, (Rules, Updates)> {
  let (input, parsed_rules) = terminated(rules_parser, line_ending)(input)?;
  let (input, parsed_updates) = updates_parser(input)?;
  Ok((input, (parsed_rules, parsed_updates)))
}

// Mini parser - Rules
fn rules_parser(input: &str) -> IResult<&str, Rules> {
  many1(terminated(
    separated_pair(complete::u32, tag("|"), complete::u32),
    line_ending,
  ))(input)
}

// Mini parser - Update sequences
fn updates_parser(input: &str) -> IResult<&str, Updates> {
  // separated_list1(separated_list1()) returns a Vec of Vecs
  separated_list1(line_ending, separated_list1(tag(","), complete::u32))(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let manual = Manual::from(input);
  let mut sum: u32 = 0;
  manual
    .updates
    .iter()
    .filter(|&update| !manual.valid_update(update))
    .map(|update| manual.fix_order(update.clone()))
    .for_each(|update| {
      let mid = update.len() / 2;
      sum += update[mid]
    });
  Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!("123", process(input)?);
    Ok(())
  }
}
