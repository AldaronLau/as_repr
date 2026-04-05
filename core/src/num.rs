//! _**`num`**_ Constant numeric operations on inherent representations

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
    F32,
    F64,
}

/// Trait indicating that a type may be compared by representation
///
/// # Safety
///
///  - The type specified in the associated constant must match the repr
pub unsafe trait Number: Copy {
    /// The representation type
    type ToRepr: Number<ToRepr = Self::ToRepr>;

    /// The type to interpret as; must be `AsRepr<Type::Variant>`
    const TYPE: Type;
    /// The minimum finite value of the representation
    const REPR_MIN: Self::ToRepr;
    /// The maximum finite value of the representation
    const REPR_MAX: Self::ToRepr;
    /// Zero in the underlying representation
    const REPR_ZERO: Self::ToRepr;
    /// One in the underlying representation
    const REPR_ONE: Self::ToRepr;
}

unsafe impl<T> Number for T
where
    T: AsReprInherent,
    T::InherentRepr: Number,
{
    type ToRepr = <T::InherentRepr as Number>::ToRepr;

    const REPR_MAX: Self::ToRepr = <T::InherentRepr as Number>::REPR_MAX;
    const REPR_MIN: Self::ToRepr = <T::InherentRepr as Number>::REPR_MIN;
    const REPR_ONE: Self::ToRepr = <T::InherentRepr as Number>::REPR_ONE;
    const REPR_ZERO: Self::ToRepr = <T::InherentRepr as Number>::REPR_ZERO;
    const TYPE: Type = <T::InherentRepr as Number>::TYPE;
}

macro_rules! num {
    ($type:ty, $name:ident) => {
        unsafe impl Number for $type {
            type ToRepr = $type;

            const REPR_MAX: Self::ToRepr = <$type>::MAX;
            const REPR_MIN: Self::ToRepr = <$type>::MIN;
            const REPR_ONE: Self::ToRepr = 1usize as _;
            const REPR_ZERO: Self::ToRepr = 0usize as _;
            const TYPE: Type = Type::$name;
        }
    };
}

num!(i8, I8);
num!(i16, I16);
num!(i32, I32);
num!(i64, I64);
num!(i128, I128);
num!(u8, U8);
num!(u16, U16);
num!(u32, U32);
num!(u64, U64);
num!(u128, U128);
num!(f32, F32);
num!(f64, F64);

/// Return true if `a` has a negative sign.
///
/// ```rust
/// # use as_repr_core::num;
/// assert!(!num::is_sign_negative(0u32));
/// assert!(!num::is_sign_negative(0.0));
/// assert!(num::is_sign_negative(-0.0));
/// assert!(num::is_sign_negative(-1i32));
/// ```
pub const fn is_sign_negative<T>(a: T) -> bool
where
    T: Number,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::F32 => unsafe { *a.cast::<f32>() }.is_sign_negative(),
        Type::F64 => unsafe { *a.cast::<f64>() }.is_sign_negative(),
        Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128 => false,
        Type::I8 => unsafe { *a.cast::<i8>() }.is_negative(),
        Type::I16 => unsafe { *a.cast::<i16>() }.is_negative(),
        Type::I32 => unsafe { *a.cast::<i32>() }.is_negative(),
        Type::I64 => unsafe { *a.cast::<i64>() }.is_negative(),
        Type::I128 => unsafe { *a.cast::<i128>() }.is_negative(),
    }
}

/// Return true if `a` has a positive sign.
///
/// ```rust
/// # use as_repr_core::num;
/// assert!(!num::is_sign_positive(0u32));
/// assert!(num::is_sign_positive(0.0));
/// assert!(!num::is_sign_positive(-0.0));
/// assert!(num::is_sign_positive(1u32));
/// ```
pub const fn is_sign_positive<T>(a: T) -> bool
where
    T: Number,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::F32 => unsafe { *a.cast::<f32>() }.is_sign_positive(),
        Type::F64 => unsafe { *a.cast::<f64>() }.is_sign_positive(),
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
