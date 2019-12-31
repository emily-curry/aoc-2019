extern crate num_traits;

pub mod permutation;

use core::ops::DivAssign;
use num_traits::{FromPrimitive, PrimInt};

pub fn digits<T>(input: T) -> Vec<T>
where
    T: FromPrimitive + PrimInt + DivAssign,
{
    let mut i = input;
    let mut d = Vec::new();
    loop {
        let digit = i % T::from_i32(10).unwrap();
        d.push(digit);
        i /= T::from_i32(10).unwrap();
        if i == T::from_i32(0).unwrap() {
            break;
        }
    }
    d
}

#[cfg(test)]
mod tests {
    use super::digits;
    use core::fmt::Debug;
    use num_traits::{FromPrimitive, PrimInt};

    #[test]
    fn digits_test() {
        assert_vec(digits(1), vec![1]);
        assert_vec(digits(0), vec![0]);
        assert_vec(digits(123), vec![3, 2, 1]);
        assert_vec(digits(9999), vec![9, 9, 9, 9]);
        assert_vec(digits(1111), vec![1, 1, 1, 1]);
        assert_vec(digits(893476), vec![6, 7, 4, 3, 9, 8]);
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
