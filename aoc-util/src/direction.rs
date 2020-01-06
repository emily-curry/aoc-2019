pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub fn clockwise(&self) -> Self {
    match self {
      Direction::Up => Direction::Right,
      Direction::Right => Direction::Down,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
    }
  }

  pub fn counter_clockwise(&self) -> Self {
    match self {
      Direction::Up => Direction::Left,
      Direction::Right => Direction::Up,
      Direction::Down => Direction::Right,
      Direction::Left => Direction::Down,
    }
  }
}
