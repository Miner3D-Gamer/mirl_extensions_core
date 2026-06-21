use crate::{ConstZero, One};

impl ConstZero for num_bigint::BigInt {
    const ZERO: Self = Self::ZERO;
}
impl One for num_bigint::BigInt {
    fn one() -> Self {
        Self::from(1)
    }
}

impl ConstZero for num_bigint::BigUint {
    const ZERO: Self = Self::ZERO;
}
impl One for num_bigint::BigUint {
    fn one() -> Self {
        Self::new(vec![1])
    }
}
