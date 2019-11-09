use std::fmt::Debug;

pub use num_traits as num;
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
pub use num_traits::{Bounded, Num, NumCast, Signed, Zero, One, Float, FloatConst};

pub trait Numeric: Bounded + Signed + Num + NumCast + Clone + Copy + PartialOrd + Debug {}

impl<T> Numeric for T where T: Bounded + Num + NumCast + Clone + Copy + Signed + PartialOrd + Debug {}

///Floating point behavioural trait
pub trait Flt: Float + FloatConst + Feq + AddAssign + SubAssign + MulAssign + DivAssign {}

impl<T> Flt for T
    where T: Float + FloatConst + Feq + AddAssign + SubAssign + MulAssign + DivAssign {}


///Compare two floating point values
pub trait Feq: Numeric {
    const EPS: Self;
    ///Compare two floating point values to nearest epsilon
    #[inline]
    fn feq_eps(self, other: Self, eps: Self) -> bool {
        self == other || ((self - other).abs() < eps)
    }

    ///Compare two floating point values
    #[inline]
    fn feq(self, other: Self) -> bool {
        Self::feq_eps(self, other, Self::EPS)
    }
}
macro_rules! impl_feq_floats {
    ($T:ty) => {
        impl Feq for $T {
            const EPS: $T = 1.0e-12;
        }
    };
}

macro_rules! impl_feq_ints {
    ($T:ty) => {
        impl Feq for $T {
            const EPS: $T = 0;
            #[inline]
            fn feq_eps(self, other: Self, _: Self) -> bool {
                self == other
            }
        }
    };
}

impl_feq_floats!(f64);
impl_feq_floats!(f32);

impl_feq_ints!(i64);
impl_feq_ints!(i32);


#[inline]
pub fn max<T>(a: T, b: T) -> T
    where
        T: Numeric,
{
    if b > a {
        b
    } else {
        a
    }
}

#[inline]
pub fn min<T>(a: T, b: T) -> T
    where
        T: Numeric,
{
    if b < a {
        b
    } else {
        a
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max() {
        assert_eq!(min(3, 4), 3);
        assert_eq!(max(3, 4), 4);
        assert_eq!(min(3.783, 0.4736624), 0.4736624);
        assert_eq!(max(-3.783, 0.4736624), 0.4736624);
        assert_eq!(min(0.4736624, 0.4736624), 0.4736624);
        assert_eq!(max(-3.783, -3.783), -3.783);
    }
}
