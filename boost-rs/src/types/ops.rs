use std::ops::*;

pub trait NumOps<Rhs = Self, Output = Self>:
Add<Rhs, Output=Output>
+ Sub<Rhs, Output=Output>
+ Mul<Rhs, Output=Output>
+ Div<Rhs, Output=Output>
+ Rem<Rhs, Output=Output>
{}

impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output=Output>
    + Sub<Rhs, Output=Output>
    + Mul<Rhs, Output=Output>
    + Div<Rhs, Output=Output>
    + Rem<Rhs, Output=Output>
{}

pub trait BitOps<Rhs = Self, Output = Self>:
Sized
+ Not<Output=Output>
+ BitXor<Output=Output>
+ BitAnd<Output=Output>
+ BitOr<Output=Output>
+ BitXorAssign
+ BitAndAssign
+ BitOrAssign
{}

impl<T, Rhs, Output> BitOps<Rhs, Output> for T where
    T: Sized
    + Not<Output=Output>
    + BitXor<Output=Output>
    + BitAnd<Output=Output>
    + BitOr<Output=Output>
    + BitXorAssign
    + BitAndAssign
    + BitOrAssign
{}

pub trait SignedBitOps<Rhs = Self, Output = Self>:
BitOps + Neg<Output=Output> {}

impl<T, Rhs, Output> SignedBitOps<Rhs, Output> for T where
    T: BitOps + Neg<Output=Output>
{}
