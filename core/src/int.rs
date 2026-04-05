//! _**`int`**_ Constant integer operations on inherent representations

use core::{
    mem::{self, ManuallyDrop},
    ptr,
};

use crate::inherent::AsReprInherent;

#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug)]
pub enum Type {
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

/// Trait indicating that a type may be compared by representation
///
/// # Safety
///
///  - The type specified in the associated constant must match the repr
pub unsafe trait Integer: Copy {
    /// The type to interpret as; must be `AsRepr<Type::Variant>`
    const TYPE: Type;
    /// The number of bits used
    const BITS: u32;

    /// The representation type
    type ToRepr: Integer<ToRepr = Self::ToRepr>;
}

unsafe impl<T> Integer for T
where
    T: AsReprInherent,
    T::InherentRepr: Integer,
{
    type ToRepr = <T::InherentRepr as Integer>::ToRepr;

    const BITS: u32 = <T::InherentRepr as Integer>::BITS;
    const TYPE: Type = <T::InherentRepr as Integer>::TYPE;
}

macro_rules! int {
    ($type:ty, $name:ident) => {
        unsafe impl Integer for $type {
            type ToRepr = $type;

            const BITS: u32 = <$type>::BITS;
            const TYPE: Type = Type::$name;
        }
    };
}

int!(i8, I8);
int!(i16, I16);
int!(i32, I32);
int!(i64, I64);
int!(i128, I128);
int!(u8, U8);
int!(u16, U16);
int!(u32, U32);
int!(u64, U64);
int!(u128, U128);

macro_rules! shift_ops {
    ($type:ty, $op:ident, $a:ident, $b:ident) => {{
        let a = unsafe { (*$a.cast::<$type>()) };
        let value = ManuallyDrop::new(a.$op($b));

        unsafe { mem::transmute_copy(&value) }
    }};
}

/// Shift left a number of bits.
///
/// ```rust
/// # use as_repr_core::int;
/// assert_eq!(int::strict_shl(5u8, 2u32), 20u8);
/// ```
pub const fn strict_shl<T>(a: T, bits: u32) -> T::ToRepr
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => shift_ops!(u8, strict_shl, a, bits),
        Type::U16 => shift_ops!(u16, strict_shl, a, bits),
        Type::U32 => shift_ops!(u32, strict_shl, a, bits),
        Type::U64 => shift_ops!(u64, strict_shl, a, bits),
        Type::U128 => shift_ops!(u128, strict_shl, a, bits),
        Type::I8 => shift_ops!(i8, strict_shl, a, bits),
        Type::I16 => shift_ops!(i16, strict_shl, a, bits),
        Type::I32 => shift_ops!(i32, strict_shl, a, bits),
        Type::I64 => shift_ops!(i64, strict_shl, a, bits),
        Type::I128 => shift_ops!(i128, strict_shl, a, bits),
    }
}

/// Shift right a number of bits.
///
/// ```rust
/// # use as_repr_core::int;
/// assert_eq!(int::strict_shr(5u8, 1u32), 2u8);
/// ```
pub const fn strict_shr<T>(a: T, bits: u32) -> T::ToRepr
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => shift_ops!(u8, strict_shr, a, bits),
        Type::U16 => shift_ops!(u16, strict_shr, a, bits),
        Type::U32 => shift_ops!(u32, strict_shr, a, bits),
        Type::U64 => shift_ops!(u64, strict_shr, a, bits),
        Type::U128 => shift_ops!(u128, strict_shr, a, bits),
        Type::I8 => shift_ops!(i8, strict_shr, a, bits),
        Type::I16 => shift_ops!(i16, strict_shr, a, bits),
        Type::I32 => shift_ops!(i32, strict_shr, a, bits),
        Type::I64 => shift_ops!(i64, strict_shr, a, bits),
        Type::I128 => shift_ops!(i128, strict_shr, a, bits),
    }
}

macro_rules! to {
    ($type:ty, $op:ident, $a:ident) => {{
        let a = unsafe { (*$a.cast::<$type>()) };
        let value = ManuallyDrop::new(a.$op());

        unsafe { mem::transmute_copy(&value) }
    }};
}

/// Convert to big endian.
pub const fn to_be<T>(a: T) -> T::ToRepr
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => to!(u8, to_be, a),
        Type::U16 => to!(u16, to_be, a),
        Type::U32 => to!(u32, to_be, a),
        Type::U64 => to!(u64, to_be, a),
        Type::U128 => to!(u128, to_be, a),
        Type::I8 => to!(i8, to_be, a),
        Type::I16 => to!(i16, to_be, a),
        Type::I32 => to!(i32, to_be, a),
        Type::I64 => to!(i64, to_be, a),
        Type::I128 => to!(i128, to_be, a),
    }
}

/// Convert to little endian.
pub const fn to_le<T>(a: T) -> T::ToRepr
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => to!(u8, to_le, a),
        Type::U16 => to!(u16, to_le, a),
        Type::U32 => to!(u32, to_le, a),
        Type::U64 => to!(u64, to_le, a),
        Type::U128 => to!(u128, to_le, a),
        Type::I8 => to!(i8, to_le, a),
        Type::I16 => to!(i16, to_le, a),
        Type::I32 => to!(i32, to_le, a),
        Type::I64 => to!(i64, to_le, a),
        Type::I128 => to!(i128, to_le, a),
    }
}

macro_rules! not {
    ($type:ty, $a:ident) => {{
        let a = unsafe { (*$a.cast::<$type>()) };
        let value = ManuallyDrop::new(!a);

        unsafe { mem::transmute_copy(&value) }
    }};
}

/// Bitwise not.
pub const fn not<T>(a: T) -> T::ToRepr
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => not!(u8, a),
        Type::U16 => not!(u16, a),
        Type::U32 => not!(u32, a),
        Type::U64 => not!(u64, a),
        Type::U128 => not!(u128, a),
        Type::I8 => not!(i8, a),
        Type::I16 => not!(i16, a),
        Type::I32 => not!(i32, a),
        Type::I64 => not!(i64, a),
        Type::I128 => not!(i128, a),
    }
}

macro_rules! from {
    ($type:ty, $op:ident, $a:ident) => {{
        let a = unsafe { (*$a.cast::<$type>()) };
        let value = ManuallyDrop::new(<$type>::$op(a));

        unsafe { mem::transmute_copy(&value) }
    }};
}

/// Convert from big endian.
pub const fn from_be<T>(a: T) -> T::ToRepr
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => from!(u8, to_be, a),
        Type::U16 => from!(u16, to_be, a),
        Type::U32 => from!(u32, to_be, a),
        Type::U64 => from!(u64, to_be, a),
        Type::U128 => from!(u128, to_be, a),
        Type::I8 => from!(i8, to_be, a),
        Type::I16 => from!(i16, to_be, a),
        Type::I32 => from!(i32, to_be, a),
        Type::I64 => from!(i64, to_be, a),
        Type::I128 => from!(i128, to_be, a),
    }
}

/// Convert from little endian.
pub const fn from_le<T>(a: T) -> T::ToRepr
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => from!(u8, from_le, a),
        Type::U16 => from!(u16, from_le, a),
        Type::U32 => from!(u32, from_le, a),
        Type::U64 => from!(u64, from_le, a),
        Type::U128 => from!(u128, from_le, a),
        Type::I8 => from!(i8, from_le, a),
        Type::I16 => from!(i16, from_le, a),
        Type::I32 => from!(i32, from_le, a),
        Type::I64 => from!(i64, from_le, a),
        Type::I128 => from!(i128, from_le, a),
    }
}

/// Return true if `a` is negative.
///
/// ```rust
/// # use as_repr_core::int;
/// assert!(!int::is_negative(0u32));
/// assert!(int::is_negative(-1i32));
/// ```
pub const fn is_negative<T>(a: T) -> bool
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128 => false,
        Type::I8 => unsafe { *a.cast::<i8>() }.is_negative(),
        Type::I16 => unsafe { *a.cast::<i16>() }.is_negative(),
        Type::I32 => unsafe { *a.cast::<i32>() }.is_negative(),
        Type::I64 => unsafe { *a.cast::<i64>() }.is_negative(),
        Type::I128 => unsafe { *a.cast::<i128>() }.is_negative(),
    }
}

/// Return true if `a` is positive.
///
/// ```rust
/// # use as_repr_core::int;
/// assert!(!int::is_positive(0u32));
/// assert!(int::is_positive(1u32));
/// ```
pub const fn is_positive<T>(a: T) -> bool
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => unsafe { *a.cast::<u8>() != 0 },
        Type::U16 => unsafe { *a.cast::<u16>() != 0 },
        Type::U32 => unsafe { *a.cast::<u32>() != 0 },
        Type::U64 => unsafe { *a.cast::<u64>() != 0 },
        Type::U128 => unsafe { *a.cast::<u128>() != 0 },
        Type::I8 => unsafe { *a.cast::<i8>() }.is_positive(),
        Type::I16 => unsafe { *a.cast::<i16>() }.is_positive(),
        Type::I32 => unsafe { *a.cast::<i32>() }.is_positive(),
        Type::I64 => unsafe { *a.cast::<i64>() }.is_positive(),
        Type::I128 => unsafe { *a.cast::<i128>() }.is_positive(),
    }
}

/// Cast an integer, panicking if out of bounds.
///
/// ```rust
/// # use as_repr_core::int;
/// assert_eq!(42i8, int::strict_cast(42u32));
/// assert_eq!(255i16, int::strict_cast(255u8));
/// ```
///
/// Panics if out of bounds:
///
/// ```rust,should_panic
/// # use as_repr_core::int;
/// int::strict_cast::<u16, u8>(256);
/// ```
///
/// ```rust,should_panic
/// # use as_repr_core::int;
/// int::strict_cast::<i8, u8>(-1);
/// ```
pub const fn strict_cast<T, U>(a: T) -> U
where
    T: Integer,
    U: Integer<ToRepr = U>,
{
    let mut output: U = unsafe { mem::zeroed() };
    let in_size = size_of::<T>();
    let out_size = size_of::<U>();
    let negative = is_negative(a);

    if negative {
        output = not(output);
    }

    if out_size > in_size {
        // If output is bigger, we don't have to worry about fitting in bounds
        let output: *mut U = &mut output;
        let a = to_le(a);

        // Copy the input into the output, leaving rest to default bits
        unsafe { ptr::copy_nonoverlapping(&a, output.cast(), 1) };
    } else {
        // If input is bigger, compare the number of used bits
        let used_bits = if negative {
            1 + T::BITS - leading_ones(a)
        } else {
            T::BITS - leading_zeros(a)
        };

        if used_bits > (out_size as u32 * 8) {
            panic!("not enough target bits for cast")
        }

        let a = to_le(a);
        let a: *const T::ToRepr = &a;

        // Copy part (or all) of the input into the output
        unsafe { ptr::copy_nonoverlapping(a.cast(), &mut output, 1) };
    }

    output = from_le(output);

    // Check if sign changed
    if is_negative(output) ^ negative {
        panic!("cannot change sign during cast")
    }

    output
}

/// Return the number of leading ones.
pub const fn leading_ones<T>(a: T) -> u32
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => unsafe { *a.cast::<u8>() }.leading_ones(),
        Type::U16 => unsafe { *a.cast::<u16>() }.leading_ones(),
        Type::U32 => unsafe { *a.cast::<u32>() }.leading_ones(),
        Type::U64 => unsafe { *a.cast::<u64>() }.leading_ones(),
        Type::U128 => unsafe { *a.cast::<u128>() }.leading_ones(),
        Type::I8 => unsafe { *a.cast::<i8>() }.leading_ones(),
        Type::I16 => unsafe { *a.cast::<i16>() }.leading_ones(),
        Type::I32 => unsafe { *a.cast::<i32>() }.leading_ones(),
        Type::I64 => unsafe { *a.cast::<i64>() }.leading_ones(),
        Type::I128 => unsafe { *a.cast::<i128>() }.leading_ones(),
    }
}

/// Return the number of leading zeros.
pub const fn leading_zeros<T>(a: T) -> u32
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => unsafe { *a.cast::<u8>() }.leading_zeros(),
        Type::U16 => unsafe { *a.cast::<u16>() }.leading_zeros(),
        Type::U32 => unsafe { *a.cast::<u32>() }.leading_zeros(),
        Type::U64 => unsafe { *a.cast::<u64>() }.leading_zeros(),
        Type::U128 => unsafe { *a.cast::<u128>() }.leading_zeros(),
        Type::I8 => unsafe { *a.cast::<i8>() }.leading_zeros(),
        Type::I16 => unsafe { *a.cast::<i16>() }.leading_zeros(),
        Type::I32 => unsafe { *a.cast::<i32>() }.leading_zeros(),
        Type::I64 => unsafe { *a.cast::<i64>() }.leading_zeros(),
        Type::I128 => unsafe { *a.cast::<i128>() }.leading_zeros(),
    }
}

/// Return the number of trailing ones.
pub const fn trailing_ones<T>(a: T) -> u32
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => unsafe { *a.cast::<u8>() }.trailing_ones(),
        Type::U16 => unsafe { *a.cast::<u16>() }.trailing_ones(),
        Type::U32 => unsafe { *a.cast::<u32>() }.trailing_ones(),
        Type::U64 => unsafe { *a.cast::<u64>() }.trailing_ones(),
        Type::U128 => unsafe { *a.cast::<u128>() }.trailing_ones(),
        Type::I8 => unsafe { *a.cast::<i8>() }.trailing_ones(),
        Type::I16 => unsafe { *a.cast::<i16>() }.trailing_ones(),
        Type::I32 => unsafe { *a.cast::<i32>() }.trailing_ones(),
        Type::I64 => unsafe { *a.cast::<i64>() }.trailing_ones(),
        Type::I128 => unsafe { *a.cast::<i128>() }.trailing_ones(),
    }
}

/// Return the number of trailing zeros.
pub const fn trailing_zeros<T>(a: T) -> u32
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => unsafe { *a.cast::<u8>() }.trailing_zeros(),
        Type::U16 => unsafe { *a.cast::<u16>() }.trailing_zeros(),
        Type::U32 => unsafe { *a.cast::<u32>() }.trailing_zeros(),
        Type::U64 => unsafe { *a.cast::<u64>() }.trailing_zeros(),
        Type::U128 => unsafe { *a.cast::<u128>() }.trailing_zeros(),
        Type::I8 => unsafe { *a.cast::<i8>() }.trailing_zeros(),
        Type::I16 => unsafe { *a.cast::<i16>() }.trailing_zeros(),
        Type::I32 => unsafe { *a.cast::<i32>() }.trailing_zeros(),
        Type::I64 => unsafe { *a.cast::<i64>() }.trailing_zeros(),
        Type::I128 => unsafe { *a.cast::<i128>() }.trailing_zeros(),
    }
}

/// Return the number of ones.
pub const fn count_ones<T>(a: T) -> u32
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => unsafe { *a.cast::<u8>() }.count_ones(),
        Type::U16 => unsafe { *a.cast::<u16>() }.count_ones(),
        Type::U32 => unsafe { *a.cast::<u32>() }.count_ones(),
        Type::U64 => unsafe { *a.cast::<u64>() }.count_ones(),
        Type::U128 => unsafe { *a.cast::<u128>() }.count_ones(),
        Type::I8 => unsafe { *a.cast::<i8>() }.count_ones(),
        Type::I16 => unsafe { *a.cast::<i16>() }.count_ones(),
        Type::I32 => unsafe { *a.cast::<i32>() }.count_ones(),
        Type::I64 => unsafe { *a.cast::<i64>() }.count_ones(),
        Type::I128 => unsafe { *a.cast::<i128>() }.count_ones(),
    }
}

/// Return the number of zeros.
pub const fn count_zeros<T>(a: T) -> u32
where
    T: Integer,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::U8 => unsafe { *a.cast::<u8>() }.count_zeros(),
        Type::U16 => unsafe { *a.cast::<u16>() }.count_zeros(),
        Type::U32 => unsafe { *a.cast::<u32>() }.count_zeros(),
        Type::U64 => unsafe { *a.cast::<u64>() }.count_zeros(),
        Type::U128 => unsafe { *a.cast::<u128>() }.count_zeros(),
        Type::I8 => unsafe { *a.cast::<i8>() }.count_zeros(),
        Type::I16 => unsafe { *a.cast::<i16>() }.count_zeros(),
        Type::I32 => unsafe { *a.cast::<i32>() }.count_zeros(),
        Type::I64 => unsafe { *a.cast::<i64>() }.count_zeros(),
        Type::I128 => unsafe { *a.cast::<i128>() }.count_zeros(),
    }
}
