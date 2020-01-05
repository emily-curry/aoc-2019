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
}
