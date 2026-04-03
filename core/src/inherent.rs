//! _**`inherent`**_ Define an inherent representation

use crate::*;

/// Similar to [`AsRepr`], but defines an "inherent" representation.
///
/// The inherent representation is a representation to use for comparisons or
/// arithmetic.
///
/// The representations should not implement this trait on themselves.
pub trait AsReprInherent: AsRepr<Self::InherentRepr> + Copy {
    /// The inherent representation
    type InherentRepr: Copy;
}
