use super::char_enum::CharEnum;
use super::point_util::points_between;
use super::SpaceMap;

#[derive(PartialEq)]
pub enum AsteroidFieldPoint {
  Space,
  Asteroid,
}

impl CharEnum for AsteroidFieldPoint {
  fn from_char(input: &char) -> Result<Self, ()> {
    match input {
      '.' => Ok(AsteroidFieldPoint::Space),
      '#' => Ok(AsteroidFieldPoint::Asteroid),
      _ => Err(()),
    }
  }

  fn to_char(&self) -> char {
    match self {
      AsteroidFieldPoint::Space => '.',
      AsteroidFieldPoint::Asteroid => '#',
    }
  }
}

impl SpaceMap<AsteroidFieldPoint> {
  pub fn max_visible(&self) -> (u16, usize, usize) {
    let asteroids = self.all_asteroids();
    let mut max = (0, 0, 0);
    for (x, y) in &asteroids {
      let visible = self.visible_asteroids(&asteroids, x, y);
      if visible > max.0 {
        max = (visible, *x, *y);
      }
    }
    max
  }

  fn all_asteroids(&self) -> Vec<(usize, usize)> {
    self.points_one_of(vec![AsteroidFieldPoint::Asteroid])
  }

  fn visible_asteroids(&self, all: &Vec<(usize, usize)>, x: &usize, y: &usize) -> u16 {
    let mut count = 0;
    for (tx, ty) in all {
      if tx == x && ty == y {
        continue;
      }
      if self.can_see((*x, *y), (*tx, *ty)) {
        count += 1;
      }
    }
    count
  }

  fn can_see(&self, source: (usize, usize), target: (usize, usize)) -> bool {
    !points_between(source, target)
      .iter()
      .fold(false, |acc, (bx, by)| {
        acc || self.at_point(*bx, *by) == &AsteroidFieldPoint::Asteroid
      })
  }
}
