/// _**`macros`**_ Define a newtype wrapper with a transparent representation
///
/// For `pub struct Wrapper(Inner)`:
///
///  - `Wrapper` implements `AsRepr<Inner>`
///  - `&Wrapper` implements `AsRepr<&Inner>`
///  - `&mut Wrapper` implements `AsRepr<&mut Inner>`
///  - `Pin<&Wrapper>` implements `AsRepr<Pin<&Inner>>`
///  - `Pin<&mut Wrapper>` implements `AsRepr<Pin<&mut Inner>>`
///
/// If the newtype is in a public API, make sure the `Inner` type is either
/// private or inaccessible so consumers cannot invalidate the newtype's
/// invariants.
///
/// # Example
///
/// ```rust
/// use std::fmt;
///
/// /// Wrapper type
/// as_repr::transparent_newtype! {
///     #[derive(Eq, PartialEq, Debug)]
///     pub struct Wrapper(i32);
/// }
///
/// impl fmt::Display for Wrapper {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         let inner: &i32 = as_repr::as_repr_ref(self);
///
///         write!(f, "{inner}")
///     }
/// }
///
/// assert_eq!(Wrapper(42).to_string(), "42");
/// ```
#[macro_export]
macro_rules! transparent_newtype {
    { $( #[ $attr:meta ] )* $vis:vis struct $newtype:ident($inner:ty); } => {
        $( #[$attr] )*
        #[repr(transparent)]
        $vis struct $newtype($inner);

        // safety: `$newtype` is `#[repr(transparent)]` referring to `$inner`
        unsafe impl $crate::AsRepr<$inner> for $newtype { }
        unsafe impl $crate::AsRepr<&$inner> for &$newtype { }
        unsafe impl $crate::AsRepr<&mut $inner> for &mut $newtype { }
        unsafe impl $crate::AsRepr<core::pin::Pin<&$inner>>
            for core::pin::Pin<&$newtype> { }
        unsafe impl $crate::AsRepr<core::pin::Pin<&mut $inner>>
            for core::pin::Pin<&mut $newtype> { }
    };
}
