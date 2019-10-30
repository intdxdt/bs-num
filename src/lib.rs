use std::fmt::{Debug, Display};
pub use num_traits::{Bounded, Num, Signed, Zero, One};

pub trait Numeric: Bounded + Signed + Num + Clone + Copy + PartialOrd + Debug {}

impl<T> Numeric for T where T: Bounded + Num + Clone + Copy + Signed + PartialOrd + Debug {}


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
