//! Auto-generated numeric range support traits.
//! Each `SupportsRangeN` trait marks numeric types capable of representing
//! the range 0..=(N-1), where N is a power of two.
//! A type supporting a larger range automatically supports every smaller range.

/// Numbers that support the number range 0-128 (2^7)
pub trait SupportsRange128 {}
/// Numbers that support the number range 0-256 (2^8)
pub trait SupportsRange256: SupportsRange128 {}
/// Numbers that support the number range 0-512 (2^9)
pub trait SupportsRange512: SupportsRange256 {}
/// Numbers that support the number range 0-1024 (2^10)
pub trait SupportsRange1024: SupportsRange512 {}
/// Numbers that support the number range 0-2048 (2^11)
pub trait SupportsRange2048: SupportsRange1024 {}
/// Numbers that support the number range 0-4096 (2^12)
pub trait SupportsRange4096: SupportsRange2048 {}
/// Numbers that support the number range 0-8192 (2^13)
pub trait SupportsRange8192: SupportsRange4096 {}
/// Numbers that support the number range 0-16384 (2^14)
pub trait SupportsRange16384: SupportsRange8192 {}
/// Numbers that support the number range 0-32768 (2^15)
pub trait SupportsRange32768: SupportsRange16384 {}
/// Numbers that support the number range 0-65536 (2^16)
pub trait SupportsRange65536: SupportsRange32768 {}

/// Numbers that support the number range 0-2147483648 (2^31)
pub trait SupportsRange2147483648: SupportsRange65536 {}
/// Numbers that support the number range 0-4294967296 (2^32)
pub trait SupportsRange4294967296: SupportsRange2147483648 {}

/// Numbers that support the number range 0-9223372036854775808 (2^63)
pub trait SupportsRange9223372036854775808: SupportsRange4294967296 {}
/// Numbers that support the number range 0-18446744073709551616 (2^64)
pub trait SupportsRange18446744073709551616: SupportsRange9223372036854775808 {}

impl<T: SupportsRange256> SupportsRange128 for T {}
impl<T: SupportsRange512> SupportsRange256 for T {}
impl<T: SupportsRange1024> SupportsRange512 for T {}
impl<T: SupportsRange2048> SupportsRange1024 for T {}
impl<T: SupportsRange4096> SupportsRange2048 for T {}
impl<T: SupportsRange8192> SupportsRange4096 for T {}
impl<T: SupportsRange16384> SupportsRange8192 for T {}
impl<T: SupportsRange32768> SupportsRange16384 for T {}
impl<T: SupportsRange65536> SupportsRange32768 for T {}
impl<T: SupportsRange2147483648> SupportsRange65536 for T {}

impl<T: SupportsRange4294967296> SupportsRange2147483648 for T {}

impl<T: SupportsRange9223372036854775808> SupportsRange4294967296 for T {}
impl<T: SupportsRange18446744073709551616> SupportsRange9223372036854775808 for T {}
