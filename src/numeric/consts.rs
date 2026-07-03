use crate::SupportsNegative;

/// The evil version of [`One`]
pub const trait NegativeOne {
    /// The value of -1 for the respective type
    fn negative_one() -> Self;
}
const impl<T: [const] One + [const] Zero + SupportsNegative + [const] core::ops::Sub<Output = T>>
    NegativeOne for T
{
    fn negative_one() -> Self {
        Self::zero() - Self::one()
    }
}

macro_rules! impl_const_one {
    ($t:ty, $v:expr) => {
        const impl One for $t {
            fn one() -> Self {
                $v
            }
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

macro_rules! impl_const_zero {
    ($t:ty, $v:expr) => {
        const impl Zero for $t {
            fn zero() -> Self {
                $v
            }
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

/// The upper and lower bound of a value
///
/// Is automatically implemented for structs that implement [`LowerBounded`] and [`UpperBounded`]
pub const trait Bounded: LowerBounded + UpperBounded {}
const impl<T: [const] LowerBounded + [const] UpperBounded> Bounded for T {}

/// The smallest value the number can represent
pub const trait LowerBounded {
    /// The smallest value the number can represent
    fn min_bound() -> Self;
}
/// The smallest value the number can represent
pub const trait UpperBounded {
    /// The smallest value the number can represent
    fn max_bound() -> Self;
}
/// Impl the upper and lower bounds in const form
#[macro_export]
macro_rules! custom_type_bounded_impl_const {
    ($t:ty, $min:expr, $max:expr) => {
        const impl LowerBounded for $t {
            fn min_bound() -> Self {
                $min
            }
        }
        const impl UpperBounded for $t {
            fn max_bound() -> Self {
                $max
            }
        }
    };
}
/// Impl the upper and lower bounds
#[macro_export]
macro_rules! custom_type_bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl LowerBounded for $t {
            fn min_bound() -> Self {
                $min
            }
        }
        impl UpperBounded for $t {
            fn max_bound() -> Self {
                $max
            }
        }
    };
}

custom_type_bounded_impl_const!(usize, usize::MIN, usize::MAX);
custom_type_bounded_impl_const!(u8, u8::MIN, u8::MAX);
custom_type_bounded_impl_const!(u16, u16::MIN, u16::MAX);
custom_type_bounded_impl_const!(u32, u32::MIN, u32::MAX);
custom_type_bounded_impl_const!(u64, u64::MIN, u64::MAX);
custom_type_bounded_impl_const!(u128, u128::MIN, u128::MAX);

custom_type_bounded_impl_const!(isize, isize::MIN, isize::MAX);
custom_type_bounded_impl_const!(i8, i8::MIN, i8::MAX);
custom_type_bounded_impl_const!(i16, i16::MIN, i16::MAX);
custom_type_bounded_impl_const!(i32, i32::MIN, i32::MAX);
custom_type_bounded_impl_const!(i64, i64::MIN, i64::MAX);
custom_type_bounded_impl_const!(i128, i128::MIN, i128::MAX);

custom_type_bounded_impl_const!(f32, f32::MIN, f32::MAX);
custom_type_bounded_impl_const!(f64, f64::MIN, f64::MAX);
custom_type_bounded_impl_const!(f128, f128::MIN, f128::MAX);
custom_type_bounded_impl_const!(f16, f16::MIN, f16::MAX);
