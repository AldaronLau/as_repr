/// _**`macros`**_ Define a newtype wrapper with a transparent representation
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
    };
}
