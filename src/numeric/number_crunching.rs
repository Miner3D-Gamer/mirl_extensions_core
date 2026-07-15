//! Stuff that may has to do with numbers but doesn't do any math

use crate::*;

/// Convert the current value into [`core::cmp::Ordering`] if self is -1, 0, or 1
pub const trait SignToOrdering {
    /// Convert the current value into [`core::cmp::Ordering`] if self is -1, 0, or 1
    fn sign_to_ordering(&self) -> core::cmp::Ordering;
}

impl<T: core::cmp::PartialOrd + Zero> SignToOrdering for T {
    #[inline(always)]
    fn sign_to_ordering(&self) -> core::cmp::Ordering {
        use core::cmp::Ordering;
        if Self::zero().lt(self) {
            Ordering::Greater
        } else if Self::zero().gt(self) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
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
const impl<T: [const] Zero + [const] core::marker::Destruct> SetZero for T {
    #[inline(always)]
    fn set_zero(&mut self) {
        *self = Self::zero();
    }
}
const impl<T: [const] One + [const] core::marker::Destruct> SetOne for T {
    #[inline(always)]
    fn set_one(&mut self) {
        *self = Self::one();
    }
}
/// Check if the value is zero
pub const trait IsZero {
    /// Check if the value is zero
    fn is_zero(&self) -> bool;
}
const impl<T: [const] PartialEq + [const] Zero + [const] core::marker::Destruct> IsZero for T {
    #[inline(always)]
    fn is_zero(&self) -> bool {
        Self::zero().eq(self)
    }
}

/// A trait for numbers to support `sign()`
pub const trait Sign {
    /// Returns the sign of the number -> -1, 0, 1
    #[must_use]
    fn sign(self) -> Self;
}

const impl<
    T: [const] NegativeOne
        + [const] Zero
        + [const] One
        + [const] PartialOrd
        + [const] core::marker::Destruct,
> Sign for T
{
    #[inline(always)]
    default fn sign(self) -> Self {
        if self > Self::zero() {
            Self::one()
        } else if self < Self::zero() {
            Self::negative_one()
        } else {
            Self::zero()
        }
    }
}

macro_rules! impl_sign_u {
    ($($t:ty),*) => {
        // Are those two spaces between the `const` and `Sign`?
        // Are those three spaces after the syntax change?
        $(const impl   $crate::Sign for $t {
            #[inline(always)]
            fn sign(self) -> Self {
                if self > 0 { 1  }
                else { 0  }
            }
        })*
    };
}

impl_sign_u!(u8, u16, u32, u64, u128, usize);
