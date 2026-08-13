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
/// If the newtype is in a public API, and also has invariants that any access
/// to `Inner` must uphold, make sure the `Inner` type is either private or
/// inaccessible so consumers cannot invalidate the newtype's invariants.
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
    {
        $( #[ $attr:meta ] )*
        $vis:vis
        struct
        $newtype:ident
        $(< $($generic:ident),+ $(,)? >)?
        ($inner:ty);
    } => {
        $( #[$attr] )*
        #[repr(transparent)]
        $vis struct $newtype $(< $($generic),* >)? ($inner);

        // safety: `$newtype` is `#[repr(transparent)]` referring to `$inner`
        unsafe impl $(< $($generic),* >)? $crate::AsRepr<$inner>
            for $newtype $(< $($generic),* >)? { }
        unsafe impl $(< $($generic),* >)? $crate::AsRepr<&$inner>
            for &$newtype $(< $($generic),* >)? { }
        unsafe impl $(< $($generic),* >)? $crate::AsRepr<&mut $inner>
            for &mut $newtype $(< $($generic),* >)? { }
        unsafe impl $(< $($generic),* >)?
            $crate::AsRepr<core::pin::Pin<&$inner>>
            for core::pin::Pin<&$newtype $(< $($generic),* >)?> { }
        unsafe impl $(< $($generic),* >)?
            $crate::AsRepr<core::pin::Pin<&mut $inner>>
            for core::pin::Pin<&mut $newtype $(< $($generic),* >)?> { }
    };
}

/// _**`macros`**_ Safely implement `AsRepr`, cascading from an existing
/// representation
///
/// If `T` is `AsRepr<U>` and `Self` is `AsRepr<T>`, then it's safe to
/// implement `AsRepr<U>` for `Self`.  This new exposed representation cascades
/// from the existing representation.
///
/// ```rust
/// use std::num::NonZero;
///
/// /// Wrapper type
/// as_repr::transparent_newtype! {
///     #[derive(Eq, PartialEq, Debug)]
///     pub struct Wrapper(i32);
/// }
///
/// as_repr::as_repr_cascading!(Wrapper as i32; Option<NonZero<i32>>, [i32; 1]);
///
/// assert_eq!(as_repr::as_repr::<i32>(Wrapper(42)), 42);
/// assert_eq!(as_repr::as_repr::<[i32; 1]>(Wrapper(42)), [42]);
/// assert_eq!(
///     as_repr::as_repr::<Option<NonZero<i32>>>(Wrapper(42)),
///     NonZero::new(42),
/// );
/// ```
#[macro_export]
macro_rules! as_repr_cascading {
    ($outer:ty as $inner:ty; $($repr:ty),* $(,)?) => {
        $(
            // safety: `$outer` is `#[repr($inner)]` and `$inner` is
            // `#[repr($repr)]`, so we can assume `$outer` is `#[repr($repr)]`
            unsafe impl $crate::AsRepr<$repr> for $outer
            where
                Self: $crate::AsRepr<$inner>,
                $inner: $crate::AsRepr<$repr>,
            { }
        )*
    };
}
