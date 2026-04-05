//! _**`float`**_ Constant float operations on inherent representations

use core::num::FpCategory;

use crate::inherent::AsReprInherent;

#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug)]
pub enum Type {
    F32,
    F64,
}

/// Trait indicating that a type may be compared by representation
///
/// # Safety
///
///  - The type specified in the associated constant must match the repr
pub unsafe trait Float: Copy {
    /// The type to interpret as; must be `AsRepr<Type::Variant>`
    const TYPE: Type;
}

unsafe impl<T> Float for T
where
    T: AsReprInherent,
    T::InherentRepr: Float,
{
    const TYPE: Type = <T::InherentRepr as Float>::TYPE;
}

macro_rules! float {
    ($type:ty, $name:ident) => {
        unsafe impl Float for $type {
            const TYPE: Type = Type::$name;
        }
    };
}

float!(f32, F32);
float!(f64, F64);

/// Classify a float.
///
/// ```rust
/// # use core::num::FpCategory;
/// # use as_repr_core::float;
/// assert_eq!(float::classify(12.4_f32), FpCategory::Normal);
/// assert_eq!(float::classify(f32::INFINITY), FpCategory::Infinite);
/// assert_eq!(float::classify(0f32), FpCategory::Zero);
/// assert_eq!(float::classify(1.1754942e-38f32), FpCategory::Subnormal);
/// assert_eq!(float::classify(f32::NAN), FpCategory::Nan);
/// ```
pub const fn classify<T>(a: T) -> FpCategory
where
    T: Float,
{
    let a: *const T = &a;

    match T::TYPE {
        Type::F32 => unsafe { *a.cast::<f32>() }.classify(),
        Type::F64 => unsafe { *a.cast::<f64>() }.classify(),
    }
}
