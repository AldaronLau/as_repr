//! Rust trait for constant #[repr(T)] conversions.

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg"
)]
#![no_std]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]
#![deny(
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    rustdoc::unescaped_backticks,
    rustdoc::redundant_explicit_links
)]

#[cfg(feature = "cmp")]
pub mod cmp;
#[cfg(feature = "float")]
pub mod float;
#[cfg(feature = "inherent")]
pub mod inherent;
#[cfg(feature = "int")]
pub mod int;
#[cfg(feature = "num")]
pub mod num;
#[cfg(feature = "ops")]
pub mod ops;

use core::{
    mem::{self, ManuallyDrop},
    num::{
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128,
        NonZeroIsize, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64,
        NonZeroU128, NonZeroUsize,
    },
    ptr::NonNull,
};

/// Trait that allows usage with [`as_repr()`]
///
/// # Safety
///
///  - For this trait to be safe to implement, it must be `#[repr(T)]`
pub unsafe trait AsRepr<T> {}

// unsafe: all types `#[repr]` themselves
unsafe impl<T> AsRepr<T> for T {}

// unsafe: `[T; 1]` and `[T]` have the same repr
unsafe impl<T> AsRepr<[T; 1]> for T {}
unsafe impl<T> AsRepr<T> for [T; 1] {}
unsafe impl<T> AsRepr<[[T; 1]; 1]> for T {}
unsafe impl<T> AsRepr<T> for [[T; 1]; 1] {}
unsafe impl<T> AsRepr<[[[T; 1]; 1]; 1]> for T {}
unsafe impl<T> AsRepr<T> for [[[T; 1]; 1]; 1] {}
unsafe impl<T> AsRepr<[[[[T; 1]; 1]; 1]; 1]> for T {}
unsafe impl<T> AsRepr<T> for [[[[T; 1]; 1]; 1]; 1] {}

// unsafe: `NonZero<T>` guaranteed to have the same layout and bit validity as T
// with the exception that that the all-zero bit pattern is invalid
// <https://doc.rust-lang.org/std/num/struct.NonZero.html#layout-1>
unsafe impl AsRepr<u8> for NonZeroU8 {}
unsafe impl AsRepr<u16> for NonZeroU16 {}
unsafe impl AsRepr<u32> for NonZeroU32 {}
unsafe impl AsRepr<u64> for NonZeroU64 {}
unsafe impl AsRepr<u128> for NonZeroU128 {}
unsafe impl AsRepr<usize> for NonZeroUsize {}
unsafe impl AsRepr<i8> for NonZeroI8 {}
unsafe impl AsRepr<i16> for NonZeroI16 {}
unsafe impl AsRepr<i32> for NonZeroI32 {}
unsafe impl AsRepr<i64> for NonZeroI64 {}
unsafe impl AsRepr<i128> for NonZeroI128 {}
unsafe impl AsRepr<isize> for NonZeroIsize {}

// unsafe: `Option<NonZero<T>>` is guaranteed to be compatible with `T`,
// including in FFI
// <https://doc.rust-lang.org/std/num/struct.NonZero.html#layout-1>
unsafe impl AsRepr<u8> for Option<NonZeroU8> {}
unsafe impl AsRepr<u16> for Option<NonZeroU16> {}
unsafe impl AsRepr<u32> for Option<NonZeroU32> {}
unsafe impl AsRepr<u64> for Option<NonZeroU64> {}
unsafe impl AsRepr<u128> for Option<NonZeroU128> {}
unsafe impl AsRepr<usize> for Option<NonZeroUsize> {}
unsafe impl AsRepr<i8> for Option<NonZeroI8> {}
unsafe impl AsRepr<i16> for Option<NonZeroI16> {}
unsafe impl AsRepr<i32> for Option<NonZeroI32> {}
unsafe impl AsRepr<i64> for Option<NonZeroI64> {}
unsafe impl AsRepr<i128> for Option<NonZeroI128> {}
unsafe impl AsRepr<isize> for Option<NonZeroIsize> {}
unsafe impl AsRepr<Option<NonZeroU8>> for u8 {}
unsafe impl AsRepr<Option<NonZeroU16>> for u16 {}
unsafe impl AsRepr<Option<NonZeroU32>> for u32 {}
unsafe impl AsRepr<Option<NonZeroU64>> for u64 {}
unsafe impl AsRepr<Option<NonZeroU128>> for u128 {}
unsafe impl AsRepr<Option<NonZeroUsize>> for usize {}
unsafe impl AsRepr<Option<NonZeroI8>> for i8 {}
unsafe impl AsRepr<Option<NonZeroI16>> for i16 {}
unsafe impl AsRepr<Option<NonZeroI32>> for i32 {}
unsafe impl AsRepr<Option<NonZeroI64>> for i64 {}
unsafe impl AsRepr<Option<NonZeroI128>> for i128 {}
unsafe impl AsRepr<Option<NonZeroIsize>> for isize {}

// unsafe: `Self::NonZeroInner` to `Option<Self::NonZeroInner>` transmute is
// sound
// <https://doc.rust-lang.org/std/num/trait.ZeroablePrimitive.html#safety>
unsafe impl AsRepr<Option<NonZeroU8>> for NonZeroU8 {}
unsafe impl AsRepr<Option<NonZeroU16>> for NonZeroU16 {}
unsafe impl AsRepr<Option<NonZeroU32>> for NonZeroU32 {}
unsafe impl AsRepr<Option<NonZeroU64>> for NonZeroU64 {}
unsafe impl AsRepr<Option<NonZeroU128>> for NonZeroU128 {}
unsafe impl AsRepr<Option<NonZeroUsize>> for NonZeroUsize {}
unsafe impl AsRepr<Option<NonZeroI8>> for NonZeroI8 {}
unsafe impl AsRepr<Option<NonZeroI16>> for NonZeroI16 {}
unsafe impl AsRepr<Option<NonZeroI32>> for NonZeroI32 {}
unsafe impl AsRepr<Option<NonZeroI64>> for NonZeroI64 {}
unsafe impl AsRepr<Option<NonZeroI128>> for NonZeroI128 {}
unsafe impl AsRepr<Option<NonZeroIsize>> for NonZeroIsize {}

// unsafe: Mutable references can be coerced to immutable
unsafe impl<T, U> AsRepr<&T> for &mut U where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<Option<&T>> for Option<&mut U> where
    U: AsRepr<T> + Sized
{
}

// unsafe: References to sized types have the same representation as pointers
unsafe impl<T, U> AsRepr<*mut T> for &U where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<*const T> for &U where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<NonNull<T>> for &U where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<Option<NonNull<T>>> for &U where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<*mut T> for &mut U where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<*const T> for &mut U where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<NonNull<T>> for &mut U where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<Option<NonNull<T>>> for &mut U where
    U: AsRepr<T> + Sized
{
}

// unsafe: Optional references to sized types have the same representation as
// pointers
// <https://doc.rust-lang.org/std/primitive.reference.html>
unsafe impl<T, U> AsRepr<*mut T> for Option<&U> where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<*const T> for Option<&U> where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<Option<NonNull<T>>> for Option<&U> where
    U: AsRepr<T> + Sized
{
}
unsafe impl<T, U> AsRepr<*mut T> for Option<&mut U> where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<*const T> for Option<&mut U> where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<Option<NonNull<T>>> for Option<&mut U> where
    U: AsRepr<T> + Sized
{
}

// unsafe: `NonNull<T>` to `Option<NonNull<T>>` transmute is sound
// <https://doc.rust-lang.org/std/ptr/struct.NonNull.html#representation>
unsafe impl<T, U> AsRepr<Option<NonNull<T>>> for NonNull<U> where
    U: AsRepr<T> + Sized
{
}

// unsafe: `Option<NonNull<T>>` has the same layout as `*const T` and `*mut T`
// https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.as_ptr
unsafe impl<T, U> AsRepr<*mut T> for Option<NonNull<U>> where
    U: AsRepr<T> + Sized
{
}
unsafe impl<T, U> AsRepr<*const T> for Option<NonNull<U>> where
    U: AsRepr<T> + Sized
{
}

// unsafe: floats are represented in bits as unsigned integers
// <https://doc.rust-lang.org/std/primitive.f32.html#method.from_bits>
unsafe impl AsRepr<u32> for f32 {}
unsafe impl AsRepr<u64> for f64 {}

// unsafe: `usize` is pointer sized (although without provenance, which is why
// this is only safe one way)
unsafe impl<T> AsRepr<usize> for *mut T where T: Sized {}
unsafe impl<T> AsRepr<usize> for *const T where T: Sized {}
unsafe impl<T> AsRepr<usize> for NonNull<T> where T: Sized {}
unsafe impl<T> AsRepr<usize> for Option<NonNull<T>> where T: Sized {}

// unsafe: `*mut T`, `*const T` and `NonNull<T>` have the same representation
unsafe impl<T, U> AsRepr<*const T> for *mut U where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<Option<NonNull<T>>> for *mut U where
    U: AsRepr<T> + Sized
{
}
unsafe impl<T, U> AsRepr<*mut T> for *const U where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<Option<NonNull<T>>> for *const U where
    U: AsRepr<T> + Sized
{
}
unsafe impl<T, U> AsRepr<*const T> for NonNull<U> where U: AsRepr<T> + Sized {}
unsafe impl<T, U> AsRepr<*mut T> for NonNull<U> where U: AsRepr<T> + Sized {}

/// Convert a type implementing [`AsRepr`] to `T`.
///
/// # Example
///
/// Types and arrays of size one have the same representation.
///
/// ```rust
/// # use as_repr_core as as_repr;
/// assert_eq!(as_repr::as_repr::<[u32; 1]>(4u32), [4u32]);
/// assert_eq!(as_repr::as_repr::<u32>([4u32]), 4u32);
///
/// assert_eq!(as_repr::as_repr::<[[u32; 1]; 1]>(4u32), [[4u32]]);
/// assert_eq!(as_repr::as_repr::<u32>([[4u32]]), 4u32);
///
/// assert_eq!(as_repr::as_repr::<[[u32; 1]; 1]>([4u32]), [[4u32]]);
/// assert_eq!(as_repr::as_repr::<[u32; 1]>([[4u32]]), [4u32]);
/// ```
pub const fn as_repr<T>(value: impl AsRepr<T>) -> T {
    let value = ManuallyDrop::new(value);

    // safety: not calling drop allows us to move the data with a "copy"
    unsafe { mem::transmute_copy(&value) }
}

/// Convert a reference to type implementing [`AsRepr`] to `&T`.
///
/// # Example
///
/// Types and arrays of size one have the same representation.
///
/// ```rust
/// # use as_repr_core as as_repr;
/// let a: &u32 = &42;
/// let b: &[u32; 1] = &[42];
///
/// assert_eq!(as_repr::as_repr_ref::<[u32; 1]>(a), b);
/// ```
pub const fn as_repr_ref<T>(value: &impl AsRepr<T>) -> &T {
    unsafe { mem::transmute(value) }
}

/// Convert a slice of a type implementing [`AsRepr`] to `&[T]`.
///
/// # Example
///
/// Types and arrays of size one have the same representation.
///
/// ```rust
/// # use as_repr_core as as_repr;
/// let a: &[u32] = &[1, 2, 3];
/// let b: &[[u32; 1]] = &[[1], [2], [3]];
///
/// assert_eq!(as_repr::as_repr_slice::<[u32; 1]>(a), b);
/// ```
pub const fn as_repr_slice<T>(value: &[impl AsRepr<T>]) -> &[T] {
    unsafe { mem::transmute(value) }
}
