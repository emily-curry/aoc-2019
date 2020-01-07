use num_traits::{FromPrimitive, PrimInt};
use std::fmt::Display;

trait Abs<T> {
  fn abs(&self) -> T;
}

impl<T> Abs<T> for T
where
  T: PrimInt + FromPrimitive,
{
  fn abs(&self) -> T {
    let v = *self;
    match v.lt(&T::from_i8(0).unwrap()) {
      true => v.mul(T::from_i8(-1).unwrap()),
      false => v,
    }
  }
}

pub trait GCD<T> {
  fn gcd(&self, b: T) -> T;
}

impl<T> GCD<T> for T
where
  T: PrimInt + FromPrimitive + Display,
{
  fn gcd(&self, b: T) -> T {
    let a = *self;
    if a == T::from_u8(0).unwrap() {
      b.abs()
    } else {
      (b % a).gcd(a)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gcd() {
    assert_eq!(3, 0.gcd(3));
    assert_eq!(3, 3.gcd(0));
    assert_eq!(1, 1.gcd(3));
    assert_eq!(2, 2.gcd(6));
    assert_eq!(3, (-9).gcd(6));
  }
}
