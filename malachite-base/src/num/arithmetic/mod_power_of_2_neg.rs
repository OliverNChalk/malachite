use crate::num::arithmetic::traits::{ModPowerOf2Neg, ModPowerOf2NegAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_power_of_2_neg<T: PrimitiveUnsigned>(x: T, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    x.wrapping_neg().mod_power_of_2(pow)
}

fn mod_power_of_2_neg_assign<T: PrimitiveUnsigned>(x: &mut T, pow: u64) {
    assert!(pow <= T::WIDTH);
    x.wrapping_neg_assign();
    x.mod_power_of_2_assign(pow);
}

macro_rules! impl_mod_power_of_2_neg {
    ($t:ident) => {
        impl ModPowerOf2Neg for $t {
            type Output = $t;

            /// Negates a number modulo another number $2^k$. Assumes the input is already reduced
            /// modulo $2^k$.
            ///
            /// $f(x, k) = y$, where $x, y < 2^k$ and $-x \equiv y \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_neg#mod_power_of_2_neg).
            #[inline]
            fn mod_power_of_2_neg(self, pow: u64) -> $t {
                mod_power_of_2_neg(self, pow)
            }
        }

        impl ModPowerOf2NegAssign for $t {
            /// Negates a number modulo another number $2^k$, in place. Assumes the input is
            /// already reduced modulo $2^k$.
            ///
            /// $x \gets y$, where $x, y < 2^k$ and $-x \equiv y \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_neg#mod_power_of_2_neg_assign).
            #[inline]
            fn mod_power_of_2_neg_assign(&mut self, pow: u64) {
                mod_power_of_2_neg_assign(self, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_neg);
