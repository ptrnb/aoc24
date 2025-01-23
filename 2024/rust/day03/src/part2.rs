use miette;
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{self, anychar},
  combinator::value,
  multi::{many1, many_till},
  sequence::{delimited, separated_pair},
  IResult, Parser,
};

#[derive(Debug, Clone)]
enum Instruction {
  Mul(u32, u32),
  Do,
  Dont,
}

#[derive(Debug, PartialEq, Eq)]
enum Processing {
  Enabled,
  Disabled,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let (_input, instructions) = parse(input).map_err(|e| miette::miette!("parse failed {}", e))?;
  let (_, result) = instructions
    .iter()
    .fold((Processing::Enabled, 0), |(process, total), op| match op {
      Instruction::Mul(a, b) if process == Processing::Enabled => (process, total + a * b),
      Instruction::Mul(_, _) => (process, total),
      Instruction::Do => (Processing::Enabled, total),
      Instruction::Dont => (Processing::Disabled, total),
    });
  Ok(result.to_string())
}

fn mul(input: &str) -> IResult<&str, Instruction> {
  let (input, _) = tag("mul")(input)?;
  let (input, pair) = delimited(
    tag("("),
    separated_pair(complete::u32, tag(","), complete::u32),
    tag(")"),
  )(input)?;
  Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
  alt((
    value(Instruction::Dont, tag("don't()")),
    value(Instruction::Do, tag("do()")),
    mul,
  ))(input)
}
fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
  many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!("48", process(input)?);
    Ok(())
  }
}
