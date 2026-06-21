use crate::SupportsNegative;

/// The evil version of [`ConstOne`]
pub trait ConstNegativeOne {
    /// The value of -1 for the respective type
    const NEGATIVE_ONE: Self;
}
impl<
    T: ConstOne + ConstZero + SupportsNegative + const core::ops::Sub<Output = T>,
> ConstNegativeOne for T
{
    const NEGATIVE_ONE: Self = Self::ZERO - Self::ONE;
}

/// A custom [`ConstOne`] as [num-traits](crate::math::ConstOne) does not support f16 or f128
pub trait ConstOne {
    /// The value of 1 in the respective type
    const ONE: Self;
}
macro_rules! impl_const_one {
    ($t:ty, $v:expr) => {
        impl ConstOne for $t {
            const ONE: Self = $v;
        }
    };
}

impl_const_one!(usize, 1);
impl_const_one!(u8, 1);
impl_const_one!(u16, 1);
impl_const_one!(u32, 1);
impl_const_one!(u64, 1);
impl_const_one!(u128, 1);

impl_const_one!(isize, 1);
impl_const_one!(i8, 1);
impl_const_one!(i16, 1);
impl_const_one!(i32, 1);
impl_const_one!(i64, 1);
impl_const_one!(i128, 1);

impl_const_one!(f16, 1.0);
impl_const_one!(f32, 1.0);
impl_const_one!(f64, 1.0);
impl_const_one!(f128, 1.0);

/// A custom [`ConstZero`] as [num-traits](crate::math::ConstZero) does not support f16 or f128
pub const trait ConstZero {
    /// The value of 0 in the respective type
    const ZERO: Self;
}
macro_rules! impl_const_zero {
    ($t:ty, $v:expr) => {
        impl ConstZero for $t {
            const ZERO: Self = $v;
        }
    };
}

impl_const_zero!(usize, 0);
impl_const_zero!(u8, 0);
impl_const_zero!(u16, 0);
impl_const_zero!(u32, 0);
impl_const_zero!(u64, 0);
impl_const_zero!(u128, 0);

impl_const_zero!(isize, 0);
impl_const_zero!(i8, 0);
impl_const_zero!(i16, 0);
impl_const_zero!(i32, 0);
impl_const_zero!(i64, 0);
impl_const_zero!(i128, 0);

impl_const_zero!(f16, 0.0);
impl_const_zero!(f32, 0.0);
impl_const_zero!(f64, 0.0);
impl_const_zero!(f128, 0.0);

/// A custom [`One`] as num-traits' does not support f16 or f128
pub const trait One {
    /// The value of 1 in the respective type
    fn one() -> Self;
}

/// A custom [`Zero`] as num-traits' does not support f16 or f128
pub const trait Zero {
    /// The value of 1 in the respective type
    fn zero() -> Self;
}
impl<T: ConstOne> const One for T {
    fn one() -> Self {
        Self::ONE
    }
}
impl<T: ConstZero> const Zero for T {
    fn zero() -> Self {
        Self::ZERO
    }
}


/// The upper and lower bound of a value
pub const trait Bounded {
    /// The minimal value this type can represent
    const MIN: Self;
    /// The maximum value this type can represent
    const MAX: Self;
}

macro_rules! bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl const Bounded for $t {
            const MIN: Self = $min;
            const MAX: Self = $max;
        }
    };
}

bounded_impl!(usize, usize::MIN, usize::MAX);
bounded_impl!(u8, u8::MIN, u8::MAX);
bounded_impl!(u16, u16::MIN, u16::MAX);
bounded_impl!(u32, u32::MIN, u32::MAX);
bounded_impl!(u64, u64::MIN, u64::MAX);
bounded_impl!(u128, u128::MIN, u128::MAX);

bounded_impl!(isize, isize::MIN, isize::MAX);
bounded_impl!(i8, i8::MIN, i8::MAX);
bounded_impl!(i16, i16::MIN, i16::MAX);
bounded_impl!(i32, i32::MIN, i32::MAX);
bounded_impl!(i64, i64::MIN, i64::MAX);
bounded_impl!(i128, i128::MIN, i128::MAX);

bounded_impl!(f32, f32::MIN, f32::MAX);
bounded_impl!(f64, f64::MIN, f64::MAX);
bounded_impl!(f128, f128::MIN, f128::MAX);
bounded_impl!(f16, f16::MIN, f16::MAX);
