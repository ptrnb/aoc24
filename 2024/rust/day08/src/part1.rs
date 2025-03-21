use std::{
  collections::{HashMap, HashSet},
  ops::{Add, Sub},
};

#[derive(Clone, Copy, Debug, Hash, Eq)]
struct Point(i32, i32);

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1
  }
}

impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self(self.0 + other.0, self.1 + other.1)
  }
}

impl Sub for Point {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self(self.0 - other.0, self.1 - other.1)
  }
}

#[derive(Debug)]
struct Rooftops {
  grid: Vec<Vec<u8>>,
  antennas: HashMap<u8, Vec<Point>>,
}

impl From<&str> for Rooftops {
  fn from(input: &str) -> Self {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    // Is it bad to nest this much when composing with iters?
    let antennas = grid
      .iter()
      .enumerate()
      .flat_map(|(x, row)| {
        row.iter().enumerate().filter_map(move |(y, &antenna)| {
          (antenna != b'.').then_some((antenna, Point(x as i32, y as i32)))
        })
      })
      .fold(
        HashMap::new(),
        |mut possible_antennas, (frequency, position)| {
          possible_antennas
            .entry(frequency)
            // insert default if empty and return mut reference to entry
            .or_insert_with(Vec::new)
            .push(position);
          possible_antennas
        },
      );

    Self { grid, antennas }
  }
}

impl Rooftops {
  fn get(&self, Point(x, y): Point) -> Option<u8> {
    self.grid.get(x as usize)?.get(y as usize).copied()
  }

  fn get_pairs(&self) -> Vec<(Point, Point)> {
    self
      .antennas
      .values()
      .flat_map(|antenna| {
        antenna
          .iter()
          .flat_map(|&p1| antenna.iter().map(move |&p2| (p1, p2)))
          .filter(|(p1, p2)| p1 != p2)
      })
      .collect()
  }

  fn signal(&self) -> HashSet<Point> {
    let mut antinodes: HashSet<Point> = HashSet::new();
    let pairs = self.get_pairs();

    for (p1, p2) in pairs {
      [p1 + (p1 - p2), p2 + (p2 - p1)]
        .into_iter()
        .filter(move |&antinode| self.get(antinode).is_some())
        .for_each(|antinode| {
          antinodes.insert(antinode);
        });
    }
    antinodes
  }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
  let rooftop_antennas = Rooftops::from(input);
  let antinodes = rooftop_antennas.signal().len();
  Ok(antinodes.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!("14", process(input)?);
    Ok(())
  }
}
