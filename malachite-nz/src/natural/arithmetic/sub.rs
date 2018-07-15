use malachite_base::num::CheckedSub;
use natural::arithmetic::sub_u32::{limbs_sub_limb_in_place, limbs_sub_limb_to_out};
use natural::Natural;
use std::fmt::Display;
use std::ops::{Sub, SubAssign};

fn sub_and_borrow(x: u32, y: u32, borrow: &mut bool) -> u32 {
    let (difference, overflow) = x.overflowing_sub(y);
    if *borrow {
        *borrow = overflow;
        let (difference, overflow) = difference.overflowing_sub(1);
        *borrow |= overflow;
        difference
    } else {
        *borrow = overflow;
        difference
    }
}

/// Interpreting a two slices of `u32`s as the limbs (in ascending order) of two `Natural`s,
/// subtracts the second from the first. Returns a pair consisting of the limbs of the result, and
/// whether there was a borrow left over; that is, whether the second `Natural` was greater than the
/// first `Natural`. The first slice must be at least as long as the second.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub;
///
/// assert_eq!(limbs_sub(&[123, 456], &[789]), (vec![4_294_966_630, 455], false));
/// assert_eq!(limbs_sub(&[123, 456], &[456, 789]), (vec![4_294_966_963, 4_294_966_962], true));
/// ```
pub fn limbs_sub(xs: &[u32], ys: &[u32]) -> (Vec<u32>, bool) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let mut difference_limbs = Vec::with_capacity(xs_len);
    let mut borrow = false;
    for i in 0..ys_len {
        difference_limbs.push(sub_and_borrow(xs[i], ys[i], &mut borrow));
    }
    if xs_len != ys_len {
        difference_limbs.extend_from_slice(&xs[ys_len..]);
        if borrow {
            borrow = limbs_sub_limb_in_place(&mut difference_limbs[ys_len..], 1);
        }
    }
    (difference_limbs, borrow)
}

/// Interpreting a two equal-length slices of `u32`s as the limbs (in ascending order) of two
/// `Natural`s, subtracts the second from the first, writing the `xs.len()` limbs of the result to
/// an output slice. Returns whether there was a borrow left over; that is, whether the second
/// `Natural` was greater than the first `Natural`. The output slice must be at least as long as
/// either input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `out_limbs` is shorter than `xs` or if `xs` and `ys` have different lengths.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_to_out;
///
/// let mut out_limbs = vec![0, 0, 0];
/// assert_eq!(limbs_sub_same_length_to_out(&mut out_limbs, &[123, 456], &[789, 123]), false);
/// assert_eq!(out_limbs, &[4_294_966_630, 332, 0]);
///
/// let mut out_limbs = vec![0, 0, 0];
/// assert_eq!(limbs_sub_same_length_to_out(&mut out_limbs, &[123, 456], &[456, 789]), true);
/// assert_eq!(out_limbs, &[4_294_966_963, 4_294_966_962, 0]);
/// ```
pub fn limbs_sub_same_length_to_out(out_limbs: &mut [u32], xs: &[u32], ys: &[u32]) -> bool {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out_limbs.len() >= len);
    let mut borrow = false;
    for i in 0..len {
        out_limbs[i] = sub_and_borrow(xs[i], ys[i], &mut borrow);
    }
    borrow
}

/// Interpreting a two slices of `u32`s as the limbs (in ascending order) of two `Natural`s,
/// subtracts the second from the first, writing the `xs.len()` limbs of the result to an output
/// slice. Returns whether there was a borrow left over; that is, whether the second `Natural` was
/// greater than the first `Natural`. The output slice must be at least as long as the first input
/// slice and the first input slice must be at least as long as the second.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `out_limbs` is shorter than `xs` or if `xs` is shorter than `ys`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_to_out;
///
/// let mut out_limbs = vec![0, 0, 0];
/// assert_eq!(limbs_sub_to_out(&mut out_limbs, &[123, 456], &[789]), false);
/// assert_eq!(out_limbs, &[4_294_966_630, 455, 0]);
///
/// let mut out_limbs = vec![0, 0, 0];
/// assert_eq!(limbs_sub_to_out(&mut out_limbs, &[123, 456], &[456, 789]), true);
/// assert_eq!(out_limbs, &[4_294_966_963, 4_294_966_962, 0]);
/// ```
pub fn limbs_sub_to_out(out_limbs: &mut [u32], xs: &[u32], ys: &[u32]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    assert!(out_limbs.len() >= xs_len);
    let borrow = limbs_sub_same_length_to_out(out_limbs, &xs[..ys_len], ys);
    if xs_len == ys_len {
        borrow
    } else if borrow {
        limbs_sub_limb_to_out(&mut out_limbs[ys_len..], &xs[ys_len..], 1)
    } else {
        out_limbs[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
        false
    }
}

/// Interpreting two equal-length slices of `u32`s as the limbs (in ascending order) of two
/// `Natural`s, subtracts the second from the first, writing the `xs.len()` limbs of the result to
/// the first (left) slice. Returns whether there was a borrow left over; that is, whether the
/// second `Natural` was greater than the first `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_same_length_in_place_left(xs, &[789, 123]), false);
/// assert_eq!(xs, &[4_294_966_630, 332]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_same_length_in_place_left(xs, &[456, 789]), true);
/// assert_eq!(xs, &[4_294_966_963, 4_294_966_962]);
/// ```
pub fn limbs_sub_same_length_in_place_left(xs: &mut [u32], ys: &[u32]) -> bool {
    let len = xs.len();
    assert_eq!(len, ys.len());
    let mut borrow = false;
    for i in 0..len {
        xs[i] = sub_and_borrow(xs[i], ys[i], &mut borrow);
    }
    borrow
}

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of two `Natural`s, subtracts
/// the second from the first, writing the `xs.len()` limbs of the result to the first (left) slice.
/// Returns whether there was a borrow left over; that is, whether the second `Natural` was greater
/// than the first `Natural`. The first slice must be at least as long as the second.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_in_place_left;
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_in_place_left(xs, &[789]), false);
/// assert_eq!(xs, &[4_294_966_630, 455]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_in_place_left(xs, &[456, 789]), true);
/// assert_eq!(xs, &[4_294_966_963, 4_294_966_962]);
/// ```
pub fn limbs_sub_in_place_left(xs: &mut [u32], ys: &[u32]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let borrow = limbs_sub_same_length_in_place_left(&mut xs[..ys_len], ys);
    if xs_len == ys_len {
        borrow
    } else if borrow {
        limbs_sub_limb_in_place(&mut xs[ys_len..], 1)
    } else {
        false
    }
}

/// Interpreting two equal-length slices of `u32`s as the limbs (in ascending order) of two
/// `Natural`s, subtracts the second from the first, writing the `xs.len()` limbs of the result to
/// the second (right) slice. Returns whether there was a borrow left over; that is, whether the
/// second `Natural` was greater than the first `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_in_place_right;
///
/// let ys = &mut [789, 123];
/// assert_eq!(limbs_sub_same_length_in_place_right(&[123, 456], ys), false);
/// assert_eq!(ys, &[4_294_966_630, 332]);
///
/// let ys = &mut [456, 789];
/// assert_eq!(limbs_sub_same_length_in_place_right(&[123, 456], ys), true);
/// assert_eq!(ys, &[4_294_966_963, 4_294_966_962]);
/// ```
pub fn limbs_sub_same_length_in_place_right(xs: &[u32], ys: &mut [u32]) -> bool {
    let len = ys.len();
    assert_eq!(xs.len(), len);
    let mut borrow = false;
    for i in 0..len {
        ys[i] = sub_and_borrow(xs[i], ys[i], &mut borrow);
    }
    borrow
}

/// Interpreting a of `u32`s and a `Vec` of `u32`s as the limbs (in ascending order) of two
/// `Natural`s, subtracts the second from the first, writing the `xs.len()` limbs of the result to
/// the `Vec`, possibly extending the `Vec`'s length. Returns whether there was a borrow left over;
/// that is, whether the second `Natural` was greater than the first `Natural`. The first slice must
/// be at least as long as the second.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len()`, m = `xs.len()` - `ys.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_in_place_right;
///
/// let mut ys = vec![789];
/// assert_eq!(limbs_sub_in_place_right(&[123, 456], &mut ys), false);
/// assert_eq!(ys, &[4_294_966_630, 455]);
///
/// let mut ys = vec![456, 789];
/// assert_eq!(limbs_sub_in_place_right(&[123, 456], &mut ys), true);
/// assert_eq!(ys, &[4_294_966_963, 4_294_966_962]);
/// ```
pub fn limbs_sub_in_place_right(xs: &[u32], ys: &mut Vec<u32>) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let borrow = limbs_sub_same_length_in_place_right(&xs[..ys_len], ys);
    if xs_len == ys_len {
        borrow
    } else {
        ys.extend_from_slice(&xs[ys_len..]);
        if borrow {
            limbs_sub_limb_in_place(&mut ys[ys_len..], 1)
        } else {
            false
        }
    }
}

fn sub_panic<S: Display, T: Display>(x: S, y: T) {
    panic!(
        "Cannot subtract a Natural from a smaller Natural. self: {}, other: {}",
        x, y
    );
}

/// Subtracts a `Natural` from a `Natural`, taking the left `Natural` by value and the right
/// `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::from(123u32) - &Natural::ZERO).to_string(), "123");
///     assert_eq!((Natural::from(456u32) - &Natural::from(123u32)).to_string(), "333");
///     assert_eq!((Natural::trillion() * 3 - &Natural::trillion()).to_string(), "2000000000000");
/// }
/// ```
impl<'a> Sub<&'a Natural> for Natural {
    type Output = Natural;

    fn sub(self, other: &'a Natural) -> Natural {
        self.checked_sub(other)
            .expect("Cannot subtract a Natural from a smaller Natural")
    }
}

/// Subtracts a `Natural` from a `Natural`, taking both `Natural`s by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(123u32) - &Natural::ZERO).to_string(), "123");
///     assert_eq!((&Natural::from(456u32) - &Natural::from(123u32)).to_string(), "333");
///     assert_eq!((&(Natural::trillion() * 3) - &Natural::trillion()).to_string(),
///         "2000000000000");
/// }
/// ```
impl<'a, 'b> Sub<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn sub(self, other: &'a Natural) -> Natural {
        self.checked_sub(other).unwrap_or_else(|| {
            sub_panic(self, other);
            unreachable!();
        })
    }
}

/// Subtracts a `Natural` from a `Natural` in place, taking the `Natural` on the RHS by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::trillion() * 10;
/// x -= &Natural::trillion();
/// x -= &(Natural::trillion() * 2);
/// x -= &(Natural::trillion() * 3);
/// x -= &(Natural::trillion() * 4);
/// assert_eq!(x.to_string(), "0");
/// ```
impl<'a> SubAssign<&'a Natural> for Natural {
    fn sub_assign(&mut self, other: &'a Natural) {
        if self.sub_assign_no_panic(other) {
            panic!("Cannot subtract a Natural from a smaller Natural");
        }
    }
}
