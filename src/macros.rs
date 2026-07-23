/// Define a newtype wrapper with a transparent representation
///
/// ```rust
/// /// Wrapper type
/// as_repr::transparent! {
///     #[derive(Eq, PartialEq, Debug)]
///     pub newtype Wrapper(i32);
/// }
/// ```
#[macro_export]
macro_rules! transparent {
    { $( #[ $attr:meta ] )* $vis:vis newtype $newtype:ident ( $inner:ident ); } => {
        $( #[$attr] )*
        #[repr(transparent)]
        $vis struct $newtype($inner);

        // safety: `$newtype` is `#[repr(transparent)]` referring to `$inner`
        unsafe impl $crate::AsRepr<$inner> for $newtype { }
    };
}
