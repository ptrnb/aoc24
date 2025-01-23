use miette;
use nom::{
  bytes::complete::tag,
  character::complete::{self, anychar},
  multi::{many0, many1, many_till},
  sequence::{delimited, separated_pair},
  IResult, Parser,
};

#[derive(Debug)]
enum Instruction {
  Mul(u32, u32),
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  // Nom IResult is not compatible with miette::Result, so have to convert the nom error using .map_err()
  let (_leftover_input, instructions) =
    parse(input).map_err(|e| miette::miette!("parse failed {}", e))?;
  let sum: u32 = instructions
    .iter()
    .map(|instruction| match instruction {
      Instruction::Mul(a, b) => a * b,
    })
    .sum();
  Ok(sum.to_string())
}

// Nom Parsers
fn instruction(input: &str) -> IResult<&str, Instruction> {
  let (input, _) = tag("mul")(input)?;
  let (input, pair) = delimited(
    tag("("),
    separated_pair(complete::u32, tag(","), complete::u32),
    tag(")"),
  )(input)?;
  Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
  many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!("161", process(input)?);
    Ok(())
  }
}
