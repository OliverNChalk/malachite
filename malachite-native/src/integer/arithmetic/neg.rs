use integer::Integer;
use malachite_base::traits::NegAssign;
use std::ops::Neg;

/// Returns the negative of an `Integer`, taking the `Integer` by value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!((-Integer::from(0)).to_string(), "0");
/// assert_eq!((-Integer::from(123)).to_string(), "-123");
/// assert_eq!((-Integer::from(-123)).to_string(), "123");
/// ```
impl Neg for Integer {
    type Output = Integer;

    fn neg(mut self) -> Integer {
        if self.abs != 0 {
            self.sign = !self.sign;
        }
        self
    }
}

/// Returns the negative of an `Integer`, taking the `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!((-&Integer::from(0)).to_string(), "0");
/// assert_eq!((-&Integer::from(123)).to_string(), "-123");
/// assert_eq!((-&Integer::from(-123)).to_string(), "123");
/// ```
impl<'a> Neg for &'a Integer {
    type Output = Integer;

    fn neg(self) -> Integer {
        if self.abs == 0 {
            Integer::from(0)
        } else {
            Integer {
                sign: !self.sign,
                abs: self.abs.clone(),
            }
        }
    }
}

/// Replaces an `Integer` with its negative.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::NegAssign;
/// use malachite_native::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(0);
///     x.neg_assign();
///     assert_eq!(x.to_string(), "0");
///
///     let mut x = Integer::from(123);
///     x.neg_assign();
///     assert_eq!(x.to_string(), "-123");
///
///     let mut x = Integer::from(-123);
///     x.neg_assign();
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl NegAssign for Integer {
    fn neg_assign(&mut self) {
        if self.abs != 0 {
            self.sign = !self.sign;
        }
    }
}
