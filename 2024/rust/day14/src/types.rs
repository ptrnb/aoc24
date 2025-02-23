use glam::IVec2;

pub const GRID_SIZE: IVec2 = if cfg!(test) {
  IVec2::new(11, 7)
} else {
  IVec2::new(101, 103)
};

#[derive(Debug)]
pub struct Robot {
  pub position: IVec2,
  pub velocity: IVec2,
}
