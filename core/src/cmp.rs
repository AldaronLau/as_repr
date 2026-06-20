//! _**`cmp`**_ Constant partial order on inherent representations

use core::{cmp::Ordering, convert::Infallible, time::Duration};

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
    Usize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    F32,
    F64,
    Duration,
    Unit,
    Never,
}

/// Trait indicating that a type may be compared by representation
///
/// # Safety
///
///  - The type specified in the associated constant must match the repr
pub unsafe trait Cmp: Copy {
    /// The type to compare as; must be `AsRepr<Type::Variant>`
    const TYPE: Type;
}

unsafe impl<T> Cmp for T
where
    T: AsReprInherent,
    T::InherentRepr: Cmp,
{
    const TYPE: Type = <T::InherentRepr as Cmp>::TYPE;
}

macro_rules! cmp {
    ($type:ty, $name:ident) => {
        unsafe impl Cmp for $type {
            const TYPE: Type = Type::$name;
        }
    };
}

cmp!(i8, I8);
cmp!(i16, I16);
cmp!(i32, I32);
cmp!(i64, I64);
cmp!(i128, I128);
cmp!(isize, Isize);
cmp!(u8, U8);
cmp!(u16, U16);
cmp!(u32, U32);
cmp!(u64, U64);
cmp!(u128, U128);
cmp!(usize, Usize);
cmp!(f32, F32);
cmp!(f64, F64);
cmp!(Duration, Duration);
cmp!((), Unit);
cmp!(Infallible, Never);

macro_rules! cmp_as {
    ($a:ident, $b:ident, $type:ty) => {{
        let (a, b) = unsafe { (*$a.cast::<$type>(), *$b.cast::<$type>()) };
        (a < b, a > b)
    }};
}

macro_rules! cmp_as_float {
    ($a:ident, $b:ident, $type:ty) => {{
        let (a, b) = unsafe { (*$a.cast::<$type>(), *$b.cast::<$type>()) };
        (a < b, a > b, a.is_nan() || b.is_nan())
    }};
}

/// Cmp `a` with `b`.
///
/// ```rust
/// # use as_repr_core::{AsRepr, inherent::AsReprInherent, cmp};
/// #[repr(transparent)]
/// #[derive(Copy, Clone)]
/// struct A(i32);
///
/// unsafe impl AsRepr<i32> for A {}
///
/// impl AsReprInherent for A {
///     type InherentRepr = i32;
/// }
///
/// assert!(cmp::partial_cmp(A(-1), A(1)).unwrap().is_lt());
/// assert!(cmp::partial_cmp(A(1), A(-1)).unwrap().is_gt());
/// assert!(cmp::partial_cmp(A(0), A(0)).unwrap().is_eq());
/// ```
pub const fn partial_cmp<T>(a: T, b: T) -> Option<Ordering>
where
    T: Cmp,
{
    const ORDERING: [[Ordering; 2]; 2] = [
        [Ordering::Equal, Ordering::Greater],
        [Ordering::Less, Ordering::Equal],
    ];

    let a: *const T = &a;
    let b: *const T = &b;
    let (less, greater) = match const { T::TYPE } {
        Type::U8 => cmp_as!(a, b, u8),
        Type::U16 => cmp_as!(a, b, u16),
        Type::U32 => cmp_as!(a, b, u32),
        Type::U64 => cmp_as!(a, b, u64),
        Type::U128 => cmp_as!(a, b, u128),
        Type::Usize => cmp_as!(a, b, usize),
        Type::I8 => cmp_as!(a, b, i8),
        Type::I16 => cmp_as!(a, b, i16),
        Type::I32 => cmp_as!(a, b, i32),
        Type::I64 => cmp_as!(a, b, i64),
        Type::I128 => cmp_as!(a, b, i128),
        Type::Isize => cmp_as!(a, b, isize),
        Type::F32 => {
            let (less, greater, impossible) = cmp_as_float!(a, b, f32);

            if impossible {
                return None;
            }

            (less, greater)
        }
        Type::F64 => {
            let (less, greater, impossible) = cmp_as_float!(a, b, f64);

            if impossible {
                return None;
            }

            (less, greater)
        }
        Type::Duration => {
            let (a, b) =
                unsafe { (*a.cast::<Duration>(), *b.cast::<Duration>()) };
            let (a_secs, b_secs) = (a.as_secs(), b.as_secs());
            let (less, greater) = (a_secs < b_secs, a_secs > b_secs);

            if less ^ greater {
                (less, greater)
            } else {
                let (a, b) = (a.subsec_nanos(), b.subsec_nanos());

                (a < b, a > b)
            }
        }
        Type::Unit => (false, false),
        Type::Never => unreachable!(),
    };

    Some(ORDERING[less as usize][greater as usize])
}

/// Cmp `a` with `b`.
///
/// ```rust
/// # use as_repr_core::{AsRepr, inherent::AsReprInherent, cmp};
/// #[repr(transparent)]
/// #[derive(Copy, Clone)]
/// struct A(i32);
///
/// unsafe impl AsRepr<i32> for A {}
///
/// impl AsReprInherent for A {
///     type InherentRepr = i32;
/// }
///
/// assert!(cmp::cmp(A(-1), A(1)).is_lt());
/// assert!(cmp::cmp(A(1), A(-1)).is_gt());
/// assert!(cmp::cmp(A(0), A(0)).is_eq());
/// ```
///
/// Fails compilation:
///
/// ```rust,compile_fail,E0080
/// # use as_repr_core::cmp;
/// assert!(cmp::cmp(1.0, 1.0).is_eq());
/// ```
pub const fn cmp<T>(a: T, b: T) -> Ordering
where
    T: Cmp,
{
    const {
        match T::TYPE {
            Type::F32 | Type::F64 => {
                panic!("cannot compare type that doesn't form a total order")
            }
            Type::U8
            | Type::U16
            | Type::U32
            | Type::U64
            | Type::U128
            | Type::Usize
            | Type::I8
            | Type::I16
            | Type::I32
            | Type::I64
            | Type::I128
            | Type::Isize
            | Type::Duration
            | Type::Unit
            | Type::Never => {}
        }
    }

    let Some(order) = partial_cmp(a, b) else {
        unreachable!()
    };

    order
}

/// Return true if `a` is NaN.
///
/// ```rust
/// # use as_repr_core::cmp;
/// assert!(cmp::is_nan(f32::NAN));
/// assert!(!cmp::is_nan(1.0));
/// assert!(!cmp::is_nan(1));
/// ```
pub const fn is_nan<T>(a: T) -> bool
where
    T: Cmp,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::F32 => unsafe { *a.cast::<f32>() }.is_nan(),
        Type::F64 => unsafe { *a.cast::<f64>() }.is_nan(),
        Type::U8
        | Type::U16
        | Type::U32
        | Type::U64
        | Type::U128
        | Type::Usize
        | Type::I8
        | Type::I16
        | Type::I32
        | Type::I64
        | Type::I128
        | Type::Isize
        | Type::Duration => false,
        Type::Unit => panic!("comparison of unit to NaN is nonsensical"),
        Type::Never => unreachable!(),
    }
}

/// Return true if `a` is zero.
///
/// ```rust
/// # use core::time::Duration;
/// # use as_repr_core::cmp;
/// assert!(cmp::is_zero(Duration::ZERO));
/// assert!(cmp::is_zero(0));
/// assert!(!cmp::is_zero(1));
/// ```
pub const fn is_zero<T>(a: T) -> bool
where
    T: Cmp,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::F32 => unsafe { *a.cast::<f32>() == 0.0 },
        Type::F64 => unsafe { *a.cast::<f64>() == 0.0 },
        Type::U8 => unsafe { *a.cast::<u8>() == 0 },
        Type::U16 => unsafe { *a.cast::<u16>() == 0 },
        Type::U32 => unsafe { *a.cast::<u32>() == 0 },
        Type::U64 => unsafe { *a.cast::<u64>() == 0 },
        Type::U128 => unsafe { *a.cast::<u128>() == 0 },
        Type::Usize => unsafe { *a.cast::<usize>() == 0 },
        Type::I8 => unsafe { *a.cast::<i8>() == 0 },
        Type::I16 => unsafe { *a.cast::<i16>() == 0 },
        Type::I32 => unsafe { *a.cast::<i32>() == 0 },
        Type::I64 => unsafe { *a.cast::<i64>() == 0 },
        Type::I128 => unsafe { *a.cast::<i128>() == 0 },
        Type::Isize => unsafe { *a.cast::<isize>() == 0 },
        Type::Duration => unsafe { *a.cast::<Duration>() }.is_zero(),
        Type::Unit => panic!("comparison of unit to zero is nonsensical"),
        Type::Never => unreachable!(),
    }
}

/// Return true if `a` is greater than `b`.
///
/// ```rust
/// # use core::time::Duration;
/// # use as_repr_core::cmp;
/// assert!(cmp::gt(Duration::new(1, 30), Duration::new(1, 29)));
/// assert!(cmp::gt(Duration::from_secs(2), Duration::from_secs(1)));
/// assert!(cmp::gt(42, 39));
/// ```
pub const fn gt<T>(a: T, b: T) -> bool
where
    T: Cmp,
{
    let Some(ordering) = partial_cmp(a, b) else {
        return false;
    };

    ordering.is_gt()
}

/// Return true if `a` is greater than or equal to `b`.
///
/// ```rust
/// # use core::time::Duration;
/// # use as_repr_core::cmp;
/// assert!(cmp::ge(Duration::new(1, 30), Duration::new(1, 29)));
/// assert!(cmp::ge(Duration::from_secs(2), Duration::from_secs(1)));
/// assert!(cmp::ge(42, 39));
///
/// assert!(cmp::ge(Duration::new(1, 300), Duration::new(1, 300)));
/// assert!(cmp::ge(Duration::from_secs(3), Duration::from_secs(3)));
/// assert!(cmp::ge(39, 39));
/// ```
pub const fn ge<T>(a: T, b: T) -> bool
where
    T: Cmp,
{
    let Some(ordering) = partial_cmp(a, b) else {
        return false;
    };

    ordering.is_ge()
}

/// Return true if `a` is less than `b`.
///
/// ```rust
/// # use core::time::Duration;
/// # use as_repr_core::cmp;
/// assert!(cmp::lt(Duration::new(1, 29), Duration::new(1, 30)));
/// assert!(cmp::lt(Duration::from_secs(1), Duration::from_secs(2)));
/// assert!(cmp::lt(39, 42));
/// ```
pub const fn lt<T>(a: T, b: T) -> bool
where
    T: Cmp,
{
    let Some(ordering) = partial_cmp(a, b) else {
        return false;
    };

    ordering.is_lt()
}

/// Return true if `a` is less than or equal to `b`.
///
/// ```rust
/// # use core::time::Duration;
/// # use as_repr_core::cmp;
/// assert!(cmp::le(Duration::new(1, 29), Duration::new(1, 30)));
/// assert!(cmp::le(Duration::from_secs(1), Duration::from_secs(2)));
/// assert!(cmp::le(39, 42));
///
/// assert!(cmp::le(Duration::new(1, 300), Duration::new(1, 300)));
/// assert!(cmp::le(Duration::from_secs(3), Duration::from_secs(3)));
/// assert!(cmp::le(39, 39));
/// ```
pub const fn le<T>(a: T, b: T) -> bool
where
    T: Cmp,
{
    let Some(ordering) = partial_cmp(a, b) else {
        return false;
    };

    ordering.is_le()
}

/// Return the minimum a `a` and `b`.
///
/// ```rust
/// # use as_repr_core::cmp;
/// assert_eq!(4.0, cmp::min(4.2, 4.0));
/// assert_eq!(4, cmp::min(4, 5));
/// ```
pub const fn min<T>(a: T, b: T) -> T
where
    T: Cmp,
{
    if is_nan(a) {
        return b;
    } else if is_nan(b) {
        return a;
    }

    let Some(ordering) = partial_cmp(a, b) else {
        unreachable!()
    };

    if ordering.is_le() { a } else { b }
}

/// Return the maximum a `a` and `b`.
///
/// ```rust
/// # use as_repr_core::cmp;
/// assert_eq!(4.2, cmp::max(4.2, 4.0));
/// assert_eq!(5, cmp::max(4, 5));
/// ```
pub const fn max<T>(a: T, b: T) -> T
where
    T: Cmp,
{
    if is_nan(a) {
        return b;
    } else if is_nan(a) {
        return a;
    }

    let Some(ordering) = partial_cmp(a, b) else {
        unreachable!()
    };

    if ordering.is_le() { b } else { a }
}
