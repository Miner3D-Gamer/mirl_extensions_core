//! Stuff that may has to do with numbers but doesn't do any math

use crate::*;

/// Convert the current value into [`core::cmp::Ordering`] if self is -1, 0, or 1
pub const trait SignToOrdering {
    /// Convert the current value into [`core::cmp::Ordering`] if self is -1, 0, or 1
    fn sign_to_ordering(&self) -> Option<core::cmp::Ordering>;
}

impl<T: ConstOne + ConstZero + ConstNegativeOne + core::cmp::PartialEq>
    SignToOrdering for T
{
    fn sign_to_ordering(&self) -> Option<core::cmp::Ordering> {
        use core::cmp::Ordering;
        Some(if Self::NEGATIVE_ONE.eq(self) {
            Ordering::Greater
        } else if Self::ZERO.eq(self) {
            Ordering::Equal
        } else if Self::ONE.eq(self) {
            Ordering::Less
        } else {
            return None;
        })
        // Some(match self {
        //     Self::NEGATIVE_ONE => Ordering::Less,
        //     Self::ZERO => Ordering::Equal,
        //     Self::ONE => Ordering::Greater,
        //     _ => return None,
        // })
    }
}

/// A trait that sets the current value to 0
pub const trait SetZero {
    /// Set the current value to 0
    fn set_zero(&mut self);
}
/// A trait that sets the current value to 1
pub const trait SetOne {
    /// Set the current value to 1
    fn set_one(&mut self);
}
impl<T: ConstZero + [const] core::marker::Destruct> const SetZero for T {
    fn set_zero(&mut self) {
        *self = Self::ZERO;
    }
}
impl<T: ConstOne + [const] core::marker::Destruct> const SetOne for T {
    fn set_one(&mut self) {
        *self = Self::ONE;
    }
}
/// Check if the value is zero
pub const trait IsZero {
    /// Check if the value is zero
    fn is_zero(&self) -> bool;
}
impl<T: [const] PartialEq + ConstZero + [const] core::marker::Destruct> const
    IsZero for T
{
    fn is_zero(&self) -> bool {
        Self::ZERO.eq(self)
    }
}

/// A trait for numbers to support `sign()`
pub const trait Sign {
    /// Returns the sign of the number -> -1, 0, 1
    #[must_use]
    fn sign(self) -> Self;
}

impl<T: ConstNegativeOne + ConstZero + ConstOne + PartialOrd> Sign for T {
    default fn sign(self) -> Self {
        if self > Self::ZERO {
            Self::ONE
        } else if self < Self::ZERO {
            Self::NEGATIVE_ONE
        } else {
            Self::ZERO
        }
    }
}

macro_rules! impl_sign_u {
    ($($t:ty),*) => {
        // See those two spaces between the `const` and `Sign`?
        $(impl const  Sign for $t {
            fn sign(self) -> Self {
                if self > 0 { 1  }
                else { 0  }
            }
        })*
    };
}

impl_sign_u!(u8, u16, u32, u64, u128, usize);
