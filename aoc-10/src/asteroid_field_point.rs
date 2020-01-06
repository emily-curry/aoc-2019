use super::char_enum::CharEnum;
use super::point_util::{points_between, ByAngle};
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
  pub fn max_visible(&self) -> (usize, usize, usize) {
    let asteroids = self.all_asteroids();
    let mut max = (0, 0, 0);
    for (x, y) in &asteroids {
      let visible = self.visible_asteroids(&asteroids, x, y).len();
      if visible > max.0 {
        max = (visible, *x, *y);
      }
    }
    max
  }

  pub fn vaporize(&mut self, source: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![];
    loop {
      let asteroids = self.all_asteroids();
      if asteroids.len() == 1 {
        break;
      }
      let mut visible = self.visible_asteroids(&asteroids, &source.0, &source.1);
      visible.sort_by_angle(&source);
      for point in &visible {
        self.set_point(point, AsteroidFieldPoint::Space);
        result.push(*point);
      }
    }
    result
  }

  fn all_asteroids(&self) -> Vec<(usize, usize)> {
    self.points_one_of(vec![AsteroidFieldPoint::Asteroid])
  }

  fn visible_asteroids(
    &self,
    all: &Vec<(usize, usize)>,
    x: &usize,
    y: &usize,
  ) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (tx, ty) in all {
      if tx == x && ty == y {
        continue;
      }
      if self.can_see((*x, *y), (*tx, *ty)) {
        result.push((*tx, *ty));
      }
    }
    result
  }

  fn can_see(&self, source: (usize, usize), target: (usize, usize)) -> bool {
    !points_between(source, target).iter().fold(false, |acc, p| {
      acc || self.at_point(p) == &AsteroidFieldPoint::Asteroid
    })
  }
}
