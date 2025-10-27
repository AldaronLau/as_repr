//! Rust trait for constant #[repr(T)] conversions.
//!
//! Uses a const trait workaround for stable Rust exposing a safe transmute to
//! the `repr` type with a trait.
//!
//! The type implementing [`AsRepr`] may have some additional invariants from
//! the repr type, but should still be usable as the underlying representation.
//!
//! # Getting Started
//!
//! ```rust
//! # use as_repr::AsRepr;
//! #[derive(Eq, PartialEq, Debug)]
//! #[repr(transparent)]
//! pub struct Id(u32);
//!
//! // safety: `Id` is `#[repr(transparent)]` referring to `u32`
//! unsafe impl AsRepr<u32> for Id { }
//!
//! assert_eq!(const { as_repr::as_repr::<u32>(Id(4)) }, 4);
//! // this is provided!
//! assert_eq!(const { as_repr::as_repr::<Id>(Id(4)) }, Id(4));
//!
//! #[repr(u32)]
//! enum IdName {
//!     Ferris = 0,
//!     Corro = 1,
//! }
//!
//! // safety: `IdName` is `#[repr(u32)]` referring to `u32`
//! unsafe impl AsRepr<u32> for IdName { }
//! // safety: `IdName` is `#[repr(u32)]`; `Id` has equivalent representation
//! unsafe impl AsRepr<Id> for IdName { }
//!
//! assert_eq!(const { as_repr::as_repr::<u32>(IdName::Ferris) }, 0);
//! assert_eq!(const { as_repr::as_repr::<u32>(IdName::Corro) }, 1);
//! assert_eq!(const { as_repr::as_repr::<Id>(IdName::Ferris) }, Id(0));
//! assert_eq!(const { as_repr::as_repr::<Id>(IdName::Corro) }, Id(1));
//! ```

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

use core::mem::{self, ManuallyDrop};

/// Trait that allows usage with [`as_repr()`]
///
/// # Safety
///
///  - For this trait to be safe to implement, it must be `#[repr(T)]`
pub unsafe trait AsRepr<T> {}

// unsafe: all types `#[repr]` themselves
unsafe impl<T> AsRepr<T> for T {}

/// Convert a type implementing [`AsRepr`] to `T`.
pub const fn as_repr<T>(value: impl AsRepr<T>) -> T {
    let value = ManuallyDrop::new(value);

    // safety: not calling drop allows us to move the data with a "copy"
    unsafe { mem::transmute_copy(&value) }
}
