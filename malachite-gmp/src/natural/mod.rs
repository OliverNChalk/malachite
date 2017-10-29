use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::*;
use std::mem;

/// A natural (non-negative) integer backed by [GMP](https://gmplib.org/).
///
/// This code uses Trevor Spiteri's
/// [`gmp_mpfr_sys`](https://tspiteri.gitlab.io/gmp-mpfr/gmp_mpfr_sys/index.html) low-level
/// bindings.
///
/// Any `Natural` small enough to fit into a `u32` is represented inline. Only integers outside this
/// range incur the costs of FFI and heap-allocation.
pub enum Natural {
    /// A small `Natural`.
    Small(u32),
    /// A large `Natural`.
    Large(mpz_t),
}

impl Natural {
    /// Creates a new `Natural` equal to 0.
    ///
    /// # Example
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// assert_eq!(Natural::new().to_string(), "0");
    /// ```
    pub fn new() -> Natural {
        Small(0)
    }

    fn demote_if_small(&mut self) {
        if let Large(x) = *self {
            if unsafe { gmp::mpz_sizeinbase(&x, 2) } <= 32 {
                *self = Small((unsafe { gmp::mpz_get_ui(&x) }) as u32)
            }
        }
    }

    fn promote(&self) -> Natural {
        match *self {
            Small(x) => unsafe {
                let mut promoted: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_si(&mut promoted, x.into());
                Large(promoted)
            },
            ref x => x.clone(),
        }
    }

    fn promote_in_place(&mut self) -> &mut mpz_t {
        if let Small(x) = *self {
            unsafe {
                let mut promoted: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut promoted, x.into());
                *self = Large(promoted);
            }
        }
        if let Large(ref mut x) = *self {
            return x;
        }
        unreachable!();
    }

    /// Returns true iff `self` is valid. To be valid, `self` cannot be negative and can only be
    /// Large when it is at least 2^(32). All Naturals used outside this crate are valid, but
    /// temporary Naturals used inside may not be.
    pub fn is_valid(&self) -> bool {
        match *self {
            Small(_) => true,
            Large(ref large) => (unsafe { gmp::mpz_cmp_ui(large, u32::max_value().into()) }) > 0,
        }
    }
}

/// Creates a default `Natural` equal to 0.
///
/// # Example
/// ```
/// use malachite_gmp::natural::Natural;
///
/// assert_eq!(Natural::default().to_string(), "0");
/// ```
impl Default for Natural {
    fn default() -> Natural {
        Small(0)
    }
}

/// If `self` is large, clears the GMP-allocated memory.
impl Drop for Natural {
    fn drop(&mut self) {
        if let Large(ref mut large) = *self {
            unsafe {
                gmp::mpz_clear(large);
            }
        }
    }
}

macro_rules! mutate_with_possible_promotion {
    ($n: ident, $small: ident, $large: ident, $process_small: expr, $process_large: expr) => {
        if let Small(ref mut $small) = *$n {
            if let Some(x) = $process_small {
                *$small = x;
                return;
            }
        }
        if let Small(x) = *$n {
            unsafe {
                let mut promoted: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut promoted, x.into());
                *$n = Large(promoted);
            }
        }
        if let Large(ref mut $large) = *$n {
            $process_large
        }
    };
}

pub mod arithmetic {
    pub mod add;
    pub mod add_u32;
    pub mod add_mul;
    pub mod add_mul_u32;
    pub mod even_odd;
    pub mod is_power_of_two;
    pub mod mul;
    pub mod mul_u32;
    pub mod neg;
    pub mod shl_u32;
    pub mod sub;
    pub mod sub_u32;
    pub mod sub_mul_u32;
}
pub mod conversion;
pub mod comparison {
    pub mod eq;
    pub mod hash;
    pub mod ord;
    pub mod partial_eq_u32;
    pub mod partial_ord_u32;
}
pub mod logic {
    pub mod assign_bit;
    pub mod clear_bit;
    pub mod flip_bit;
    pub mod from_limbs;
    pub mod get_bit;
    pub mod limb_count;
    pub mod limbs;
    pub mod not;
    pub mod set_bit;
    pub mod significant_bits;
    pub mod trailing_zeros;
}
