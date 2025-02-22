use glam::I64Vec2;

pub type Button = I64Vec2;
pub type Prize = I64Vec2;

#[derive(Debug)]
pub struct ClawMachine {
  pub a: Button,
  pub b: Button,
  pub prize: Prize,
}
