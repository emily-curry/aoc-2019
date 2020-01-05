use core::ops::DivAssign;
use num_traits::{FromPrimitive, PrimInt};

pub trait ToDigits<T> {
  fn digits(&self) -> Vec<T>;
}

impl<T> ToDigits<T> for T
where
  T: FromPrimitive + PrimInt + DivAssign,
{
  fn digits(&self) -> Vec<T> {
    let mut i = *self;
    let mut d = Vec::new();
    loop {
      let digit = i % T::from_u8(10).unwrap();
      d.push(digit);
      i /= T::from_u8(10).unwrap();
      if i == T::from_u8(0).unwrap() {
        break;
      }
    }
    d
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use core::fmt::Debug;
  use num_traits::{FromPrimitive, PrimInt};

  #[test]
  fn digits_test() {
    assert_vec(1.digits(), vec![1]);
    assert_vec(0.digits(), vec![0]);
    assert_vec(123.digits(), vec![3, 2, 1]);
    assert_vec(9999.digits(), vec![9, 9, 9, 9]);
    assert_vec(1111.digits(), vec![1, 1, 1, 1]);
    assert_vec(893476.digits(), vec![6, 7, 4, 3, 9, 8]);
  }

  fn assert_vec<T>(output: Vec<T>, required: Vec<T>)
  where
    T: PrimInt + FromPrimitive + Debug,
  {
    if output.len() != required.len() {
      panic!();
    }
    let mut i = 0;
    while i < output.len() {
      assert_eq!(output[i], required[i]);
      i += 1;
    }
  }
}
