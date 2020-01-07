use super::GCD;
use num_traits::{FromPrimitive, PrimInt};
use std::fmt::Display;

pub trait LCM<T> {
  fn lcm(&self, b: T) -> T;
}

impl<T> LCM<T> for T
where
  T: PrimInt + FromPrimitive + Display,
{
  fn lcm(&self, b: T) -> T {
    let a = *self;
    (a * b) / a.gcd(b)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lcm() {
    assert_eq!(12, 4.lcm(6));
    assert_eq!(20, 4.lcm(5));
    assert_eq!(42, 21.lcm(6).lcm(7));
  }
}
