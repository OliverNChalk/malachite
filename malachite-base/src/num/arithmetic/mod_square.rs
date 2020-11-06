use num::arithmetic::traits::{
    ModPow, ModPowAssign, ModPowPrecomputed, ModPowPrecomputedAssign, ModSquare, ModSquareAssign,
    ModSquarePrecomputed, ModSquarePrecomputedAssign,
};

macro_rules! impl_mod_square {
    ($t:ident) => {
        impl ModSquare for $t {
            type Output = $t;

            /// Computes `self.square()` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModSquare;
            ///
            /// assert_eq!(2u8.mod_square(10), 4);
            /// assert_eq!(100u32.mod_square(497), 60);
            /// ```
            #[inline]
            fn mod_square(self, m: $t) -> $t {
                self.mod_pow(2, m)
            }
        }

        impl ModSquareAssign for $t {
            /// Replaces `self` with `self.square()` mod `m`. Assumes the input is already reduced
            /// mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
            ///
            /// let mut n = 2u8;
            /// n.mod_square_assign(10);
            /// assert_eq!(n, 4);
            ///
            /// let mut n = 100u32;
            /// n.mod_square_assign(497);
            /// assert_eq!(n, 60);
            /// ```
            #[inline]
            fn mod_square_assign(&mut self, m: $t) {
                self.mod_pow_assign(2, m);
            }
        }

        impl ModSquarePrecomputed<u64, $t> for $t {
            /// Computes `self.square()` mod `m`. Assumes the input is already reduced mod `m`. Some
            /// precomputed data is provided; this speeds up computations involving several modular
            /// squarings with the same modulus. The precomputed data should be obtained using
            /// `precompute_mod_pow_data`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::{
            ///     ModPowPrecomputed, ModSquarePrecomputed
            /// };
            ///
            /// let data = u16::precompute_mod_pow_data(&497);
            /// assert_eq!(100u16.mod_square_precomputed(497, &data), 60);
            /// assert_eq!(200u16.mod_square_precomputed(497, &data), 240);
            /// assert_eq!(300u16.mod_square_precomputed(497, &data), 43);
            /// ```
            #[inline]
            fn mod_square_precomputed(self, m: $t, data: &Self::Data) -> Self::Output {
                self.mod_pow_precomputed(2, m, data)
            }
        }

        impl ModSquarePrecomputedAssign<u64, $t> for $t {
            /// Replaces `self` with `self.square()` mod `m`. Assumes the input is already reduced
            /// mod `m`. Some precomputed data is provided; this speeds up computations involving
            /// several modular squarings with the same modulus. The precomputed data should be
            /// obtained using `precompute_mod_pow_data`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::{
            ///     ModPowPrecomputed, ModSquarePrecomputedAssign
            /// };
            ///
            /// let data = u32::precompute_mod_pow_data(&497);
            ///
            /// let mut x = 100u32;
            /// x.mod_square_precomputed_assign(497, &data);
            /// assert_eq!(x, 60);
            ///
            /// let mut x = 200u32;
            /// x.mod_square_precomputed_assign(497, &data);
            /// assert_eq!(x, 240);
            ///
            /// let mut x = 300u32;
            /// x.mod_square_precomputed_assign(497, &data);
            /// assert_eq!(x, 43);
            /// ```
            #[inline]
            fn mod_square_precomputed_assign(&mut self, m: $t, data: &Self::Data) {
                self.mod_pow_precomputed_assign(2, m, data);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_square);
