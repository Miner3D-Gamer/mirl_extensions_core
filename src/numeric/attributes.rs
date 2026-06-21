use mirl_core::impl_empty_trait;

/// Implemented for types that support negative numbers
pub trait SupportsNegative {}
impl_empty_trait!(SupportsNegative for i8, i16, i32, i64, i128, f16, f32, f64, f128);

/// If the given type supports values between 0 and 1
pub trait SupportsDecimalRange0To1 {}

/// Numbers that support the number range 0-127 (2^8)/2
pub trait SupportsRange128 {}
/// Numbers that support the number range 0-255 (2^8)
pub trait SupportsRange256 {}
/// Numbers that support the number range 0-32767 (2^16)/2
pub trait SupportsRange32768 {}
/// Numbers that support the number range 0-65535 (2^16)
pub trait SupportsRange65536 {}

mirl_core::impl_empty_trait!(SupportsDecimalRange0To1 for f16 ,f32 ,f64 ,f128);

mirl_core::impl_empty_trait!(SupportsRange128 for u8, u16, u32, u64 ,u128 ,usize ,i8 ,i16 ,i32, i64, i128, isize ,f16 ,f32 ,f64 ,f128);

mirl_core::impl_empty_trait!(SupportsRange256 for u8, u16, u32 ,u64 ,u128 ,usize, i16, i32 ,i64 ,i128, isize, f16, f32, f64 ,f128);

mirl_core::impl_empty_trait!(SupportsRange32768 for u16, u32 ,u64 ,u128 ,usize ,i16 ,i32 ,i64 ,i128, isize, f16, f32, f64 ,f128);

mirl_core::impl_empty_trait!(SupportsRange65536 for u16, u32 ,u64 ,u128 ,usize, i32 ,i64 ,i128, isize, f16, f32, f64 ,f128);

/// If a given struct is a number type, regardless
pub trait IsNumberType {}

mirl_core::impl_empty_trait!(IsNumberType for u8, u16, u32, u64 ,u128 ,usize ,i8 ,i16 ,i32, i64, i128, isize ,f16 ,f32 ,f64 ,f128);
