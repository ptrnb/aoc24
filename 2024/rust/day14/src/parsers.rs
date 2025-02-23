use glam::IVec2;

use nom::{
  self,
  bytes::complete::tag,
  character::complete::{self, line_ending},
  multi::separated_list1,
  sequence::{preceded, separated_pair},
  IResult, Parser,
};

use crate::types::Robot;

pub fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
  let (remaining, result) = separated_list1(line_ending, robot)(input)?;
  Ok((remaining, result))
}

fn robot(input: &str) -> IResult<&str, Robot> {
  let (input, (pos, speed)) = separated_pair(position, tag(" "), velocity)(input)?;
  Ok((
    input,
    Robot {
      position: pos,
      velocity: speed,
    },
  ))
}

fn position(input: &str) -> IResult<&str, IVec2> {
  preceded(
    tag("p="),
    separated_pair(complete::i32, tag(","), complete::i32).map(|(x, y)| IVec2::new(x, y)),
  )(input)
}

fn velocity(input: &str) -> IResult<&str, IVec2> {
  preceded(
    tag("v="),
    separated_pair(complete::i32, tag(","), complete::i32).map(|(x, y)| IVec2::new(x, y)),
  )(input)
}
