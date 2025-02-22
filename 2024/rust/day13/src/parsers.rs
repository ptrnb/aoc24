use nom::{
  bytes::complete::tag,
  character::complete::{self, line_ending},
  multi::separated_list1,
  sequence::{preceded, separated_pair, terminated, tuple},
  IResult, Parser,
};

use crate::types::{Button, ClawMachine, Prize};

fn a_button(input: &str) -> IResult<&str, Button> {
  preceded(
    tag("Button A: X+"),
    separated_pair(complete::i64, tag(", Y+"), complete::i64).map(|(x, y)| Button::new(x, y)),
  )(input)
}

fn b_button(input: &str) -> IResult<&str, Button> {
  preceded(
    tag("Button B: X+"),
    separated_pair(complete::i64, tag(", Y+"), complete::i64).map(|(x, y)| Button::new(x, y)),
  )(input)
}

fn prize(input: &str) -> IResult<&str, Prize> {
  preceded(
    tag("Prize: X="),
    separated_pair(complete::i64, tag(", Y="), complete::i64).map(|(x, y)| Prize::new(x, y)),
  )(input)
}

fn machine(input: &str) -> IResult<&str, ClawMachine> {
  let (input, (a, b, prize)) = tuple((
    terminated(a_button, line_ending),
    terminated(b_button, line_ending),
    prize,
  ))(input)?;
  Ok((input, ClawMachine { a, b, prize }))
}

pub fn parse(input: &str) -> IResult<&str, Vec<ClawMachine>> {
  let result = separated_list1(tuple((line_ending, line_ending)), machine)(input)?;
  Ok(result)
}
