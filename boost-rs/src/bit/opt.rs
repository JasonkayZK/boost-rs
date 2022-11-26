#![allow(clippy::manual_swap)]

use std::ops;

use crate::types::integer::{Integer, SignedInteger};

pub fn swap<T>(mut x: T, mut y: T) -> (T, T)
    where T: ops::BitXor<Output=T> + ops::BitXorAssign + Copy {
    x ^= y;
    y ^= x;
    x ^= y;
    (x, y)
}

pub fn add_one<T>(x: T) -> T
    where T: SignedInteger {
    -(!x)
}

pub fn sub_one<T>(x: T) -> T
    where T: SignedInteger {
    !(-x)
}

pub fn neg<T>(x: T) -> T
    where T: SignedInteger + ops::Add<i32, Output=T> {
    !x + 1
}

pub fn hamming_weight<T>(_x: T) -> T
    where T: Integer {
    _x
}

#[cfg(test)]
mod tests {
    use crate::bit::opt::*;

    #[test]
    fn test_swap() {
        let x = 5;
        let y = 3;

        assert_eq!(swap(x, y), (y, x));
    }

    #[test]
    fn test_add_one() {
        let x = rand::random::<i32>();
        assert_eq!(add_one(x), x + 1);
    }

    #[test]
    fn test_sub_one() {
        let x = rand::random::<i32>();
        assert_eq!(sub_one(x), x - 1);
    }

    #[test]
    fn test_neg() {
        let x = rand::random::<i32>();
        assert_eq!(neg(x), -x);
    }
}
