use crate::{One, Zero};

const impl Zero for num_bigint::BigInt {
    fn zero() -> Self {
        Self::ZERO
    }
}
impl One for num_bigint::BigInt {
    fn one() -> Self {
        Self::from(1)
    }
}

const impl Zero for num_bigint::BigUint {
    fn zero() -> Self {
        Self::ZERO
    }
}

impl One for num_bigint::BigUint {
    fn one() -> Self {
        Self::new(vec![1])
    }
}
