//! _**`ops`**_ Constant arithmetic operations on inherent representations

use core::{
    mem::{self, ManuallyDrop},
    time::Duration,
};

use crate::inherent::AsReprInherent;

#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug)]
pub enum SaturatingAddSubType {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    Duration,
}

/// Trait indicating that a type may be added as its representation
///
/// # Safety
///
///  - The type specified in the associated constant must match the repr
pub unsafe trait SaturatingAdd: Copy {
    /// The type to operate as; must be `AsRepr<SaturatingAddSub::Variant>`
    const TYPE: SaturatingAddSubType;

    /// The representation type
    type ToRepr;
}

unsafe impl<T> SaturatingAdd for T
where
    T: AsReprInherent,
    T::InherentRepr: SaturatingAdd,
{
    type ToRepr = <T::InherentRepr as SaturatingAdd>::ToRepr;

    const TYPE: SaturatingAddSubType = <T::InherentRepr as SaturatingAdd>::TYPE;
}

/// Trait indicating that a type may be subtracted as its representation
///
/// # Safety
///
///  - The type specified in the associated constant must match the repr
pub unsafe trait SaturatingSub: Copy {
    /// The type to operate as; must be `AsRepr<SaturatingAddSubType::Variant>`
    const TYPE: SaturatingAddSubType;

    /// The representation type
    type ToRepr;
}

unsafe impl<T> SaturatingSub for T
where
    T: AsReprInherent,
    T::InherentRepr: SaturatingSub,
{
    type ToRepr = <T::InherentRepr as SaturatingSub>::ToRepr;

    const TYPE: SaturatingAddSubType = <T::InherentRepr as SaturatingSub>::TYPE;
}

macro_rules! saturating_add_sub {
    ($type:ty, $name:ident) => {
        unsafe impl SaturatingAdd for $type {
            type ToRepr = $type;

            const TYPE: SaturatingAddSubType = SaturatingAddSubType::$name;
        }

        unsafe impl SaturatingSub for $type {
            type ToRepr = $type;

            const TYPE: SaturatingAddSubType = SaturatingAddSubType::$name;
        }
    };
}

saturating_add_sub!(i8, I8);
saturating_add_sub!(i16, I16);
saturating_add_sub!(i32, I32);
saturating_add_sub!(i64, I64);
saturating_add_sub!(i128, I128);
saturating_add_sub!(u8, U8);
saturating_add_sub!(u16, U16);
saturating_add_sub!(u32, U32);
saturating_add_sub!(u64, U64);
saturating_add_sub!(u128, U128);
saturating_add_sub!(Duration, Duration);

#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug)]
pub enum SaturatingMulDivType {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
}

/// Trait indicating that a type may be multiplied as its representation
///
/// # Safety
///
///  - The type specified in the associated constant must match the repr
pub unsafe trait SaturatingMul: Copy {
    /// The type to operate as; must be `AsRepr<SaturatingMulDiv::Variant>`
    const TYPE: SaturatingMulDivType;

    /// The representation type
    type ToRepr;
}

unsafe impl<T> SaturatingMul for T
where
    T: AsReprInherent,
    T::InherentRepr: SaturatingMul,
{
    type ToRepr = <T::InherentRepr as SaturatingMul>::ToRepr;

    const TYPE: SaturatingMulDivType = <T::InherentRepr as SaturatingMul>::TYPE;
}

/// Trait indicating that a type may be divided as its representation
///
/// # Safety
///
///  - The type specified in the associated constant must match the repr
pub unsafe trait SaturatingDiv: Copy {
    /// The type to operate as; must be `AsRepr<SaturatingMulDivType::Variant>`
    const TYPE: SaturatingMulDivType;

    /// The representation type
    type ToRepr;
}

unsafe impl<T> SaturatingDiv for T
where
    T: AsReprInherent,
    T::InherentRepr: SaturatingDiv,
{
    type ToRepr = <T::InherentRepr as SaturatingDiv>::ToRepr;

    const TYPE: SaturatingMulDivType = <T::InherentRepr as SaturatingDiv>::TYPE;
}

macro_rules! saturating_mul_div {
    ($type:ty, $name:ident) => {
        unsafe impl SaturatingMul for $type {
            type ToRepr = $type;

            const TYPE: SaturatingMulDivType = SaturatingMulDivType::$name;
        }

        unsafe impl SaturatingDiv for $type {
            type ToRepr = $type;

            const TYPE: SaturatingMulDivType = SaturatingMulDivType::$name;
        }
    };
}

saturating_mul_div!(i8, I8);
saturating_mul_div!(i16, I16);
saturating_mul_div!(i32, I32);
saturating_mul_div!(i64, I64);
saturating_mul_div!(i128, I128);
saturating_mul_div!(u8, U8);
saturating_mul_div!(u16, U16);
saturating_mul_div!(u32, U32);
saturating_mul_div!(u64, U64);
saturating_mul_div!(u128, U128);

macro_rules! saturating_ops {
    ($type:ty, $op:ident, $a:ident, $b:ident) => {{
        let (a, b) = unsafe { (*$a.cast::<$type>(), *$b.cast::<$type>()) };
        let value = ManuallyDrop::new(a.$op(b));

        unsafe { mem::transmute_copy(&value) }
    }};
}

/// Add two values together.
///
/// ```rust
/// # use as_repr_core::ops;
/// assert_eq!(ops::saturating_add(4u8, 8u8), 12u8);
/// assert_eq!(ops::saturating_add(250u8, 250u8), 255u8);
/// ```
pub const fn saturating_add<T>(a: T, b: T) -> T::ToRepr
where
    T: SaturatingAdd,
{
    use SaturatingAddSubType as Type;

    let a: *const T = &a;
    let b: *const T = &b;

    match T::TYPE {
        Type::U8 => saturating_ops!(u8, saturating_add, a, b),
        Type::U16 => saturating_ops!(u16, saturating_add, a, b),
        Type::U32 => saturating_ops!(u32, saturating_add, a, b),
        Type::U64 => saturating_ops!(u64, saturating_add, a, b),
        Type::U128 => saturating_ops!(u128, saturating_add, a, b),
        Type::I8 => saturating_ops!(i8, saturating_add, a, b),
        Type::I16 => saturating_ops!(i16, saturating_add, a, b),
        Type::I32 => saturating_ops!(i32, saturating_add, a, b),
        Type::I64 => saturating_ops!(i64, saturating_add, a, b),
        Type::I128 => saturating_ops!(i128, saturating_add, a, b),
        Type::Duration => saturating_ops!(Duration, saturating_add, a, b),
    }
}

/// Subtract a value from another.
///
/// ```rust
/// # use as_repr_core::ops;
/// assert_eq!(ops::saturating_sub(4u32, 8u32), 0u32);
/// assert_eq!(ops::saturating_sub(250u32, 200u32), 50u32);
/// ```
pub const fn saturating_sub<T>(a: T, b: T) -> T::ToRepr
where
    T: SaturatingSub,
{
    use SaturatingAddSubType as Type;

    let a: *const T = &a;
    let b: *const T = &b;

    match T::TYPE {
        Type::U8 => saturating_ops!(u8, saturating_sub, a, b),
        Type::U16 => saturating_ops!(u16, saturating_sub, a, b),
        Type::U32 => saturating_ops!(u32, saturating_sub, a, b),
        Type::U64 => saturating_ops!(u64, saturating_sub, a, b),
        Type::U128 => saturating_ops!(u128, saturating_sub, a, b),
        Type::I8 => saturating_ops!(i8, saturating_sub, a, b),
        Type::I16 => saturating_ops!(i16, saturating_sub, a, b),
        Type::I32 => saturating_ops!(i32, saturating_sub, a, b),
        Type::I64 => saturating_ops!(i64, saturating_sub, a, b),
        Type::I128 => saturating_ops!(i128, saturating_sub, a, b),
        Type::Duration => saturating_ops!(Duration, saturating_sub, a, b),
    }
}

/// Multiply two values together.
///
/// ```rust
/// # use as_repr_core::ops;
/// assert_eq!(ops::saturating_mul(4u8, 8u8), 32u8);
/// assert_eq!(ops::saturating_mul(250u8, 250u8), 255u8);
/// ```
pub const fn saturating_mul<T>(a: T, b: T) -> T::ToRepr
where
    T: SaturatingMul,
{
    use SaturatingMulDivType as Type;

    let a: *const T = &a;
    let b: *const T = &b;

    match T::TYPE {
        Type::U8 => saturating_ops!(u8, saturating_mul, a, b),
        Type::U16 => saturating_ops!(u16, saturating_mul, a, b),
        Type::U32 => saturating_ops!(u32, saturating_mul, a, b),
        Type::U64 => saturating_ops!(u64, saturating_mul, a, b),
        Type::U128 => saturating_ops!(u128, saturating_mul, a, b),
        Type::I8 => saturating_ops!(i8, saturating_mul, a, b),
        Type::I16 => saturating_ops!(i16, saturating_mul, a, b),
        Type::I32 => saturating_ops!(i32, saturating_mul, a, b),
        Type::I64 => saturating_ops!(i64, saturating_mul, a, b),
        Type::I128 => saturating_ops!(i128, saturating_mul, a, b),
    }
}

/// Divide a value by another.
///
/// ```rust
/// # use as_repr_core::ops;
/// assert_eq!(ops::saturating_div(5u8, 2u8), 2u8);
/// assert_eq!(ops::saturating_div(i32::MAX, -1), i32::MIN + 1);
/// assert_eq!(ops::saturating_div(i32::MIN, -1), i32::MAX);
/// ```
pub const fn saturating_div<T>(a: T, b: T) -> T::ToRepr
where
    T: SaturatingDiv,
{
    use SaturatingMulDivType as Type;

    let a: *const T = &a;
    let b: *const T = &b;

    match T::TYPE {
        Type::U8 => saturating_ops!(u8, saturating_div, a, b),
        Type::U16 => saturating_ops!(u16, saturating_div, a, b),
        Type::U32 => saturating_ops!(u32, saturating_div, a, b),
        Type::U64 => saturating_ops!(u64, saturating_div, a, b),
        Type::U128 => saturating_ops!(u128, saturating_div, a, b),
        Type::I8 => saturating_ops!(i8, saturating_div, a, b),
        Type::I16 => saturating_ops!(i16, saturating_div, a, b),
        Type::I32 => saturating_ops!(i32, saturating_div, a, b),
        Type::I64 => saturating_ops!(i64, saturating_div, a, b),
        Type::I128 => saturating_ops!(i128, saturating_div, a, b),
    }
}
