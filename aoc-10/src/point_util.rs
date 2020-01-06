use aoc_util::GCD;

pub fn points_between(source: (usize, usize), target: (usize, usize)) -> Vec<(usize, usize)> {
  let sx = source.0 as isize;
  let sy = source.1 as isize;
  let tx = target.0 as isize;
  let ty = target.1 as isize;
  let dx = tx - sx;
  let dy = ty - sy;
  let mut result = vec![];
  let c = dx.gcd(dy);

  let bx = dx / c;
  let by = dy / c;
  for m in 1..c {
    let cx = (bx * m) + sx;
    let cy = (by * m) + sy;
    result.push((cx as usize, cy as usize));
  }

  result
}

pub trait AngleBetween {
  fn angle(&self, target: &(usize, usize)) -> f64;
}

impl AngleBetween for (usize, usize) {
  fn angle(&self, target: &(usize, usize)) -> f64 {
    let dx = target.0 as f64 - self.0 as f64;
    let dy = target.1 as f64 - self.1 as f64;
    let ng = dy.atan2(dx) + std::f64::consts::FRAC_PI_2;
    if ng < 0.0 {
      ng + std::f64::consts::PI * 2.0
    } else {
      ng
    }
  }
}

pub trait ByAngle {
  fn sort_by_angle(&mut self, source: &(usize, usize));
}

impl ByAngle for Vec<(usize, usize)> {
  fn sort_by_angle(&mut self, source: &(usize, usize)) {
    self.sort_unstable_by(|p1, p2| source.angle(p1).partial_cmp(&source.angle(p2)).unwrap());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_points() {
    assert_eq!(vec![(3, 2), (4, 2)], points_between((2, 2), (5, 2)));
    assert_eq!(vec![(4, 2)], points_between((2, 1), (6, 3)));
    assert_eq!(vec![(2, 4)], points_between((1, 2), (3, 6)));
    assert_eq!(
      vec![(4, 2), (5, 3), (6, 4),],
      points_between((3, 1), (7, 5))
    );
    assert_eq!(Vec::<(usize, usize)>::new(), points_between((0, 1), (7, 2)));
    assert_eq!(vec![(7, 2), (14, 3)], points_between((0, 1), (21, 4)));
    assert_eq!(
      vec![(3, 3), (6, 5), (9, 7), (12, 9), (15, 11), (18, 13)],
      points_between((0, 1), (21, 15))
    );
    assert_eq!(vec![(3, 2)], points_between((4, 4), (2, 0)))
  }

  #[test]
  fn test_angle() {
    assert_eq!(0.0, (1, 1).angle(&(1, 0)));
    assert_eq!(std::f64::consts::FRAC_PI_2, (1, 1).angle(&(2, 1)));
    assert_eq!(std::f64::consts::PI, (1, 1).angle(&(1, 2)));
    assert_eq!(3.0 * std::f64::consts::FRAC_PI_2, (1, 1).angle(&(0, 1)));
  }

  #[test]
  fn test_angle_by() {
    assert_eq!(vec![(1, 0), (2, 1), (1, 2), (0, 1)], {
      let mut unsorted = vec![(0, 1), (2, 1), (1, 0), (1, 2)];
      unsorted.sort_by_angle(&(1, 1));
      unsorted
    });
  }
}
