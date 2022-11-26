use crate::empty_trait_impl;
use crate::types::ops::{NumOps, SignedBitOps};

pub trait Integer: Sized + PartialOrd + Ord + PartialEq + Eq + NumOps {}

pub trait SignedInteger: Integer + SignedBitOps {}

empty_trait_impl!(Integer for usize u8 u16 u32 u64 u128);

empty_trait_impl!(Integer for isize i8 i16 i32 i64 i128);

empty_trait_impl!(SignedInteger for isize i8 i16 i32 i64 i128);

#[cfg(test)]
mod tests {
    #[test]
    fn test_integer() {}
}
