use num::arithmetic::traits::{ArithmeticCheckedShl, DivisibleByPowerOf2, ModPowerOf2, ShrRound};
use num::basic::integers::PrimitiveInt;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::{
    ExactFrom, ExactInto, IntegerMantissaAndExponent, RawMantissaAndExponent,
    SciMantissaAndExponent, WrappingFrom,
};
use num::float::PrimitiveFloat;
use num::logic::traits::{BitAccess, LeadingZeros, LowMask, SignificantBits, TrailingZeros};
use rounding_modes::RoundingMode;

fn _raw_mantissa_and_exponent<T: PrimitiveFloat>(x: T) -> (u64, u64) {
    let bits = x.to_bits();
    let mantissa = bits.mod_power_of_2(T::MANTISSA_WIDTH);
    let exponent: u64 = (bits >> T::MANTISSA_WIDTH).exact_into();
    let exponent = exponent.mod_power_of_2(T::EXPONENT_WIDTH);
    (mantissa, exponent)
}

#[inline]
fn _raw_mantissa<T: PrimitiveFloat>(x: T) -> u64 {
    x.to_bits().mod_power_of_2(T::MANTISSA_WIDTH)
}

#[inline]
fn _raw_exponent<T: PrimitiveFloat>(x: T) -> u64 {
    let exponent: u64 = (x.to_bits() >> T::MANTISSA_WIDTH).exact_into();
    exponent.mod_power_of_2(T::EXPONENT_WIDTH)
}

fn _from_raw_mantissa_and_exponent<T: PrimitiveFloat>(raw_mantissa: u64, raw_exponent: u64) -> T {
    assert!(raw_mantissa.significant_bits() <= T::MANTISSA_WIDTH);
    assert!(raw_exponent.significant_bits() <= T::EXPONENT_WIDTH);
    let x = T::from_bits((raw_exponent << T::MANTISSA_WIDTH) | raw_mantissa);
    // Only output the canonical NaN
    if x.is_nan() {
        T::NAN
    } else {
        x
    }
}

fn _integer_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(x: T) -> (u64, i64) {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (mut raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    if raw_exponent == 0 {
        let trailing_zeros = raw_mantissa.trailing_zeros();
        (
            raw_mantissa >> trailing_zeros,
            i64::wrapping_from(trailing_zeros) + T::MIN_EXPONENT,
        )
    } else {
        raw_mantissa.set_bit(T::MANTISSA_WIDTH);
        let trailing_zeros = TrailingZeros::trailing_zeros(raw_mantissa);
        (
            raw_mantissa >> trailing_zeros,
            i64::wrapping_from(raw_exponent + trailing_zeros) + T::MIN_EXPONENT - 1,
        )
    }
}

fn _integer_mantissa_primitive_float<T: PrimitiveFloat>(x: T) -> u64 {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (mut raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    if raw_exponent != 0 {
        raw_mantissa.set_bit(T::MANTISSA_WIDTH);
    }
    raw_mantissa >> raw_mantissa.trailing_zeros()
}

fn _integer_exponent_primitive_float<T: PrimitiveFloat>(x: T) -> i64 {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    if raw_exponent == 0 {
        i64::wrapping_from(raw_mantissa.trailing_zeros()) + T::MIN_EXPONENT
    } else {
        i64::wrapping_from(
            raw_exponent
                + if raw_mantissa == 0 {
                    T::MANTISSA_WIDTH
                } else {
                    TrailingZeros::trailing_zeros(raw_mantissa)
                },
        ) + T::MIN_EXPONENT
            - 1
    }
}

fn _from_integer_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    integer_mantissa: u64,
    integer_exponent: i64,
) -> Option<T> {
    if integer_mantissa == 0 {
        return Some(T::ZERO);
    }
    let trailing_zeros = integer_mantissa.trailing_zeros();
    let (integer_mantissa, adjusted_exponent) = (
        integer_mantissa >> trailing_zeros,
        integer_exponent + i64::wrapping_from(trailing_zeros),
    );
    let mantissa_bits = integer_mantissa.significant_bits();
    let sci_exponent = adjusted_exponent.checked_add(i64::exact_from(mantissa_bits))? - 1;
    let mut raw_mantissa;
    let raw_exponent;
    if sci_exponent < T::MIN_EXPONENT || sci_exponent > T::MAX_EXPONENT {
        return None;
    } else if sci_exponent < T::MIN_NORMAL_EXPONENT {
        if adjusted_exponent < T::MIN_EXPONENT {
            return None;
        } else {
            raw_exponent = 0;
            raw_mantissa = integer_mantissa << (adjusted_exponent - T::MIN_EXPONENT);
        }
    } else if mantissa_bits > T::MANTISSA_WIDTH + 1 {
        return None;
    } else {
        raw_exponent = u64::exact_from(sci_exponent + i64::low_mask(T::EXPONENT_WIDTH - 1));
        raw_mantissa = integer_mantissa << (T::MANTISSA_WIDTH + 1 - mantissa_bits);
        raw_mantissa.clear_bit(T::MANTISSA_WIDTH);
    }
    Some(T::from_raw_mantissa_and_exponent(
        raw_mantissa,
        raw_exponent,
    ))
}

fn _sci_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(x: T) -> (T, i64) {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    if raw_exponent == 0 {
        let leading_zeros =
            LeadingZeros::leading_zeros(raw_mantissa) - (u64::WIDTH - T::MANTISSA_WIDTH);
        let mut mantissa = raw_mantissa << (leading_zeros + 1);
        mantissa.clear_bit(T::MANTISSA_WIDTH);
        (
            T::from_raw_mantissa_and_exponent(mantissa, u64::wrapping_from(T::MAX_EXPONENT)),
            i64::wrapping_from(T::MANTISSA_WIDTH - leading_zeros - 1) + T::MIN_EXPONENT,
        )
    } else {
        (
            T::from_raw_mantissa_and_exponent(raw_mantissa, u64::wrapping_from(T::MAX_EXPONENT)),
            i64::wrapping_from(raw_exponent) - T::MAX_EXPONENT,
        )
    }
}

fn _sci_mantissa_primitive_float<T: PrimitiveFloat>(x: T) -> T {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (mut mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    if raw_exponent == 0 {
        mantissa <<= LeadingZeros::leading_zeros(mantissa) - (u64::WIDTH - T::MANTISSA_WIDTH) + 1;
        mantissa.clear_bit(T::MANTISSA_WIDTH);
    }
    T::from_raw_mantissa_and_exponent(mantissa, u64::wrapping_from(T::MAX_EXPONENT))
}

fn _sci_exponent_primitive_float<T: PrimitiveFloat>(x: T) -> i64 {
    assert!(x.is_finite());
    assert!(x != T::ZERO);
    let (raw_mantissa, raw_exponent) = x.raw_mantissa_and_exponent();
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    if raw_exponent == 0 {
        i64::wrapping_from(u64::WIDTH - LeadingZeros::leading_zeros(raw_mantissa) - 1)
            + T::MIN_EXPONENT
    } else {
        i64::wrapping_from(raw_exponent) - T::MAX_EXPONENT
    }
}

#[allow(clippy::wrong_self_convention)]
fn _from_sci_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    sci_mantissa: T,
    sci_exponent: i64,
) -> Option<T> {
    assert!(sci_mantissa.is_finite());
    assert!(sci_mantissa > T::ZERO);
    if sci_exponent < T::MIN_EXPONENT || sci_exponent > T::MAX_EXPONENT {
        return None;
    }
    let (mut orig_mantissa, orig_exponent) = sci_mantissa.raw_mantissa_and_exponent();
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    if orig_exponent != u64::wrapping_from(T::MAX_EXPONENT) {
        return None;
    }
    if sci_exponent < T::MIN_NORMAL_EXPONENT {
        let shift = T::MIN_NORMAL_EXPONENT - sci_exponent;
        if orig_mantissa.divisible_by_power_of_2(u64::wrapping_from(shift)) {
            orig_mantissa.set_bit(T::MANTISSA_WIDTH);
            Some(T::from_raw_mantissa_and_exponent(orig_mantissa >> shift, 0))
        } else {
            None
        }
    } else {
        Some(T::from_raw_mantissa_and_exponent(
            orig_mantissa,
            u64::wrapping_from(sci_exponent + T::MAX_EXPONENT),
        ))
    }
}

/// Returns the scientific mantissa and exponent.
///
/// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and $m_s$ is
/// a rational number with $1 \leq m_s < 2$. We represent the rational mantissa as a float. The
/// conversion might not be exact, so we round to the nearest float using the provided rounding
/// mode. If the rounding mode is `Exact` but the conversion is not exact, `None` is returned.
/// $$
/// f(x, r) \approx (\frac{x}{2^{\lfloor \log_2 x \rfloor}}, \lfloor \log_2 x \rfloor).
/// $$
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
/// use malachite_base::num::conversion::mantissa_and_exponent::*;
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::float::PrimitiveFloat;
/// use malachite_base::rounding_modes::RoundingMode;
///
/// fn test<T: PrimitiveUnsigned, U: PrimitiveFloat>(
///     n: T,
///     rm: RoundingMode,
///     out: Option<(U, u64)>
/// ) {
///     assert_eq!(
///         sci_mantissa_and_exponent_with_rounding(n, rm).map(|(m, e)| (NiceFloat(m), e)),
///         out.map(|(m, e)| (NiceFloat(m), e))
///     );
/// }
/// test::<u32, f32>(3, RoundingMode::Down, Some((1.5, 1)));
/// test::<u32, f32>(3, RoundingMode::Ceiling, Some((1.5, 1)));
/// test::<u32, f32>(3, RoundingMode::Up, Some((1.5, 1)));
/// test::<u32, f32>(3, RoundingMode::Nearest, Some((1.5, 1)));
/// test::<u32, f32>(3, RoundingMode::Exact, Some((1.5, 1)));
///
/// test::<u32, f32>(123, RoundingMode::Floor, Some((1.921875, 6)));
/// test::<u32, f32>(123, RoundingMode::Down, Some((1.921875, 6)));
/// test::<u32, f32>(123, RoundingMode::Ceiling, Some((1.921875, 6)));
/// test::<u32, f32>(123, RoundingMode::Up, Some((1.921875, 6)));
/// test::<u32, f32>(123, RoundingMode::Nearest, Some((1.921875, 6)));
/// test::<u32, f32>(123, RoundingMode::Exact, Some((1.921875, 6)));
///
/// test::<u32, f32>(1000000000, RoundingMode::Nearest, Some((1.8626451, 29)));
/// ```
pub fn sci_mantissa_and_exponent_with_rounding<T: PrimitiveUnsigned, U: PrimitiveFloat>(
    x: T,
    rm: RoundingMode,
) -> Option<(U, u64)> {
    assert_ne!(x, T::ZERO);
    let significant_bits = x.significant_bits();
    let mut exponent = significant_bits - 1;
    let mut raw_mantissa: u64 = if significant_bits > U::MANTISSA_WIDTH {
        let shift = significant_bits - U::MANTISSA_WIDTH - 1;
        if rm == RoundingMode::Exact && TrailingZeros::trailing_zeros(x) < shift {
            return None;
        }
        x.shr_round(shift, rm).wrapping_into()
    } else {
        let x: u64 = x.wrapping_into();
        x << (U::MANTISSA_WIDTH - significant_bits + 1)
    };
    if raw_mantissa.significant_bits() == U::MANTISSA_WIDTH + 2 {
        // Rounding up to a power of 2
        raw_mantissa >>= 1;
        exponent += 1;
    }
    raw_mantissa.clear_bit(U::MANTISSA_WIDTH);
    // Note that Self::MAX_EXPONENT is also the raw exponent of 1.0.
    Some((
        U::from_raw_mantissa_and_exponent(raw_mantissa, u64::wrapping_from(U::MAX_EXPONENT)),
        exponent,
    ))
}

/// Constructs a primitive integer from its scientific mantissa and exponent.
///
/// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and $m_s$ is a
/// rational number with $1 \leq m_s < 2$. Here, the rational mantissa is provided as a float. If
/// the mantissa is outside the range $[1, 2)$, `None` is returned.
///
/// Some combinations of mantissas and exponents do not specify an integer, in which case the
/// resulting value is rounded to an integer using the specified rounding mode. If the rounding
/// mode is `Exact` but the input does not exactly specify an integer, `None` is returned.
///
/// $$
/// f(x, r) \approx 2^{e_s}m_s.
/// $$
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `sci_mantissa` is zero.
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
/// use malachite_base::num::conversion::mantissa_and_exponent::*;
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_base::num::float::PrimitiveFloat;
/// use malachite_base::rounding_modes::RoundingMode;
/// use std::str::FromStr;
///
/// fn test<T: PrimitiveUnsigned, U: PrimitiveFloat>(
///     mantissa: U,
///     exponent: u64,
///     rm: RoundingMode,
///     out: Option<T>
/// ) {
///     assert_eq!(
///         from_sci_mantissa_and_exponent_with_rounding::<T, U>(mantissa, exponent, rm),
///         out
///     );
/// };
/// test::<u32, f32>(1.5, 1, RoundingMode::Floor, Some(3));
/// test::<u32, f32>(1.5, 1, RoundingMode::Down, Some(3));
/// test::<u32, f32>(1.5, 1, RoundingMode::Ceiling, Some(3));
/// test::<u32, f32>(1.5, 1, RoundingMode::Up, Some(3));
/// test::<u32, f32>(1.5, 1, RoundingMode::Nearest, Some(3));
/// test::<u32, f32>(1.5, 1, RoundingMode::Exact, Some(3));
///
/// test::<u32, f32>(1.51, 1, RoundingMode::Floor, Some(3));
/// test::<u32, f32>(1.51, 1, RoundingMode::Down, Some(3));
/// test::<u32, f32>(1.51, 1, RoundingMode::Ceiling, Some(4));
/// test::<u32, f32>(1.51, 1, RoundingMode::Up, Some(4));
/// test::<u32, f32>(1.51, 1, RoundingMode::Nearest, Some(3));
/// test::<u32, f32>(1.51, 1, RoundingMode::Exact, None);
///
/// test::<u32, f32>(2.0, 1, RoundingMode::Floor, None);
/// test::<u32, f32>(10.0, 1, RoundingMode::Floor, None);
/// test::<u32, f32>(0.5, 1, RoundingMode::Floor, None);
/// ```
pub fn from_sci_mantissa_and_exponent_with_rounding<T: PrimitiveUnsigned, U: PrimitiveFloat>(
    sci_mantissa: U,
    sci_exponent: u64,
    rm: RoundingMode,
) -> Option<T> {
    assert_ne!(sci_mantissa, U::ZERO);
    if sci_mantissa < U::ONE || sci_mantissa >= U::TWO {
        return None;
    }
    let mut raw_mantissa = sci_mantissa.raw_mantissa();
    raw_mantissa.set_bit(U::MANTISSA_WIDTH);
    if sci_exponent >= U::MANTISSA_WIDTH {
        T::checked_from(raw_mantissa)?.arithmetic_checked_shl(sci_exponent - U::MANTISSA_WIDTH)
    } else {
        let shift = U::MANTISSA_WIDTH - sci_exponent;
        if rm == RoundingMode::Exact && TrailingZeros::trailing_zeros(raw_mantissa) < shift {
            return None;
        }
        T::checked_from(raw_mantissa.shr_round(shift, rm))
    }
}

macro_rules! impl_mantissa_and_exponent_unsigned {
    ($t:ident) => {
        impl IntegerMantissaAndExponent<$t, u64> for $t {
            /// Returns the integer mantissa and exponent.
            ///
            /// When $x$ is nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
            /// $m_i$ is an odd integer.
            /// $$
            /// f(x) = (\frac{|x|}{2^{e_i}}, e_i),
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// The inverse operation is `from_integer_mantissa_and_exponent`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn integer_mantissa_and_exponent(self) -> ($t, u64) {
                assert_ne!(self, 0);
                let exponent = TrailingZeros::trailing_zeros(self);
                (self >> exponent, exponent)
            }

            /// Returns the integer mantissa.
            ///
            /// When $x$ is nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
            /// $m_i$ is an odd integer.
            /// $$
            /// f(x) = \frac{|x|}{2^{e_i}},
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn integer_mantissa(self) -> $t {
                assert_ne!(self, 0);
                self >> self.trailing_zeros()
            }

            /// Returns the integer exponent.
            ///
            /// When $x$ is nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
            /// $m_i$ is an odd integer.
            /// $$
            /// f(x) = e_i,
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn integer_exponent(self) -> u64 {
                assert_ne!(self, 0);
                TrailingZeros::trailing_zeros(self)
            }

            /// Constructs a primitive unsigned integer from its integer mantissa and exponent.
            ///
            /// When $x$ is nonzero, we can write $x = 2^{e_i}m_i$, where $e_i$ is an integer and
            /// $m_i$ is an odd integer.
            ///
            /// $$
            /// f(x) = 2^{e_i}m_i,
            /// $$
            /// or `None` if the result cannot be exactly represented as an integer of the desired
            /// type (this happens if the exponent is too large).
            ///
            /// The input does not have to be reduced; that is, the mantissa does not have to be
            /// odd.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn from_integer_mantissa_and_exponent(
                integer_mantissa: $t,
                integer_exponent: u64,
            ) -> Option<$t> {
                integer_mantissa.arithmetic_checked_shl(integer_exponent)
            }
        }
    };
}
apply_to_unsigneds!(impl_mantissa_and_exponent_unsigned);

macro_rules! impl_sci_mantissa_and_exponent_unsigned {
    ($u:ident) => {
        macro_rules! impl_sci_mantissa_and_exponent_unsigned_inner {
            ($f:ident) => {
                impl SciMantissaAndExponent<$f, u64> for $u {
                    /// Returns the scientific mantissa and exponent.
                    ///
                    /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an
                    /// integer and $m_s$ is a rational number with $1 \leq m_s < 2$. We represent
                    /// the rational mantissa as a float. The conversion might not be exact, so we
                    /// round to the nearest float using the `Nearest` rounding mode. To use other
                    /// rounding modes, use `sci_mantissa_and_exponent`.
                    ///
                    /// If the result cannot be expressed as an integer of the specified type,
                    /// `None` is returned.
                    /// $$
                    /// f(x) \approx (\frac{x}{2^{\lfloor \log_2 x \rfloor}},
                    /// \lfloor \log_2 x \rfloor).
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `self` is zero.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::conversion::mantissa_and_exponent`
                    /// module.
                    #[inline]
                    fn sci_mantissa_and_exponent(self) -> ($f, u64) {
                        sci_mantissa_and_exponent_with_rounding(self, RoundingMode::Nearest)
                            .unwrap()
                    }

                    /// Constructs a primitive integer from its scientific mantissa and exponent.
                    ///
                    /// When $x$ is positive, we can write $x = 2^{e_s}m_s$, where $e_s$ is an
                    /// integer and $m_s$ is a rational number with $1 \leq m_s < 2$. Here, the
                    /// rational mantissa is provided as a float. If the mantissa is outside the
                    /// range $[1, 2)$, `None` is returned.
                    ///
                    /// Some combinations of mantissas and exponents do not specify an integer, in
                    /// which case the resulting value is rounded to an integer using the `Nearest`
                    /// rounding mode. To specify other rounding modes, use
                    /// `from_sci_mantissa_and_exponent_with_rounding`.
                    ///
                    /// $$
                    /// f(x) \approx 2^{e_s}m_s.
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `sci_mantissa` is zero.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::conversion::mantissa_and_exponent`
                    /// module.
                    #[inline]
                    fn from_sci_mantissa_and_exponent(
                        sci_mantissa: $f,
                        sci_exponent: u64,
                    ) -> Option<$u> {
                        from_sci_mantissa_and_exponent_with_rounding(
                            sci_mantissa,
                            sci_exponent,
                            RoundingMode::Nearest,
                        )
                    }
                }
            };
        }
        apply_to_primitive_floats!(impl_sci_mantissa_and_exponent_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_sci_mantissa_and_exponent_unsigned);

macro_rules! impl_mantissa_and_exponent_primitive_float {
    ($t:ident) => {
        impl RawMantissaAndExponent<u64, u64> for $t {
            /// Returns the raw mantissa and exponent.
            ///
            /// The raw exponent and raw mantissa are the actual bit patterns used to represent the
            /// components of `self`. When `self` is nonzero and finite, the raw exponent $e_r$ is
            /// an integer in $[0, 2^E-2]$ and the raw mantissa $m_r$ is an integer in
            /// $[0, 2^M-1]$.
            ///
            /// When `self` is nonzero and finite, $f(x) = (m_r, e_r)$, where
            /// $$
            /// m_r = \\begin{cases}
            ///     2^{M+2^{E-1}-2}|x| & |x| < 2^{2-2^{E-1}} \\\\
            ///     2^M \left ( \frac{|x|}{2^{\lfloor \log_2 |x| \rfloor}}-1\right ) &
            ///     \textrm{otherwise}
            /// \\end{cases}
            /// $$
            /// and
            /// $$
            /// e_r = \\begin{cases}
            ///     0 & |x| < 2^{2-2^{E-1}} \\\\
            ///     \lfloor \log_2 |x| \rfloor + 2^{E-1} - 1 & \textrm{otherwise}.
            /// \\end{cases}
            /// $$
            /// and $M$ and $E$ are the mantissa width and exponent width, respectively.
            ///
            /// For zeros, infinities, or `NaN`, refer to IEEE 754 or look at the examples below.
            ///
            /// The inverse operation is `from_raw_mantissa_and_exponent`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn raw_mantissa_and_exponent(self) -> (u64, u64) {
                _raw_mantissa_and_exponent(self)
            }

            /// Returns the raw mantissa.
            ///
            /// The raw mantissa is the actual bit pattern used to represent the mantissa of
            /// `self`. When `self` is nonzero and finite, it is an integer in $[0, 2^M-1]$.
            ///
            /// When `self` is nonzero and finite,
            /// $$
            /// f(x) = \\begin{cases}
            ///     2^{M+2^{E-1}-2}|x| & |x| < 2^{2-2^{E-1}} \\\\
            ///     2^M \left ( \frac{|x|}{2^{\lfloor \log_2 |x| \rfloor}}-1\right )
            ///     & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            /// where $M$ and $E$ are the mantissa width and exponent width, respectively.
            ///
            /// For zeros, infinities, or `NaN`, refer to IEEE 754 or look at the examples below.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn raw_mantissa(self) -> u64 {
                _raw_mantissa(self)
            }

            /// Returns the raw exponent.
            ///
            /// The raw exponent is the actual bit pattern used to represent the exponent of
            /// `self`. When `self` is nonzero and finite, it is an integer in $[0, 2^E-2]$.
            ///
            /// When `self` is nonzero and finite,
            /// $$
            /// f(x) = \\begin{cases}
            ///     0 & |x| < 2^{2-2^{E-1}} \\\\
            ///     \lfloor \log_2 |x| \rfloor + 2^{E-1} - 1 & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            /// where $M$ and $E$ are the mantissa width and exponent width, respectively.
            ///
            /// For zeros, infinities, or `NaN`, refer to IEEE 754 or look at the examples below.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn raw_exponent(self) -> u64 {
                _raw_exponent(self)
            }

            /// Constructs a float from its raw mantissa and exponent.
            ///
            /// The raw exponent and raw mantissa are the actual bit patterns used to represent the
            /// components of a float. When the float is nonzero and finite, the raw exponent $e_r$
            /// is an integer in $[0, 2^E-2]$ and the raw mantissa $m_r$ is an integer in
            /// $[0, 2^M-1]$.
            ///
            /// When the exponent is not `2^E-1`,
            /// $$
            /// f(m_r, e_r) = \\begin{cases}
            ///     2^{2-2^{E-1}-M}m_r & e_r = 0 \\\\
            ///     2^{e_r-2^{E-1}+1}(2^{-M}m_r+1) & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            /// where $M$ and $E$ are the mantissa width and exponent width, respectively.
            ///
            /// For zeros, infinities, or `NaN`, refer to IEEE 754 or look at the examples below.
            ///
            /// This function only outputs a single, canonical `NaN`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn from_raw_mantissa_and_exponent(raw_mantissa: u64, raw_exponent: u64) -> $t {
                _from_raw_mantissa_and_exponent(raw_mantissa, raw_exponent)
            }
        }

        impl IntegerMantissaAndExponent<u64, i64> for $t {
            /// Returns the integer mantissa and exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_i}m_i$, where
            /// $e_i$ is an integer and $m_i$ is an odd integer.
            /// $$
            /// f(x) = (\frac{|x|}{2^{e_i}}, e_i),
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// The inverse operation is `from_integer_mantissa_and_exponent`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn integer_mantissa_and_exponent(self) -> (u64, i64) {
                _integer_mantissa_and_exponent_primitive_float(self)
            }

            /// Returns the integer mantissa.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_i}m_i$, where
            /// $e_i$ is an integer and $m_i$ is an odd integer.
            /// $$
            /// f(x) = \frac{|x|}{2^{e_i}},
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn integer_mantissa(self) -> u64 {
                _integer_mantissa_primitive_float(self)
            }

            /// Returns the integer exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_i}m_i$, where
            /// $e_i$ is an integer and $m_i$ is an odd integer.
            /// $$
            /// f(x) = e_i,
            /// $$
            /// where $e_i$ is the unique integer such that $x/2^{e_i}$ is an odd integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn integer_exponent(self) -> i64 {
                _integer_exponent_primitive_float(self)
            }

            /// Constructs a float from its integer mantissa and exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_i}m_i$, where
            /// $e_i$ is an integer and $m_i$ is an odd integer.
            ///
            /// $$
            /// f(x) = 2^{e_i}m_i,
            /// $$
            /// or `None` if the result cannot be exactly represented as a float of the desired
            /// type (this happens if the exponent is too large or too small, or if the mantissa's
            /// precision is too high for the exponent).
            ///
            /// The input does not have to be reduced; that is, the mantissa does not have to be
            /// odd.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn from_integer_mantissa_and_exponent(
                integer_mantissa: u64,
                integer_exponent: i64,
            ) -> Option<$t> {
                _from_integer_mantissa_and_exponent_primitive_float(
                    integer_mantissa,
                    integer_exponent,
                )
            }
        }

        impl SciMantissaAndExponent<$t, i64> for $t {
            /// Returns the scientific mantissa and exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_s}m_s$, where
            /// $e_s$ is an integer and $m_s$ is a rational number with $1 \leq m_s < 2$. If $x$ is
            /// a valid float, the scientific mantissa $m_s$ is always exactly representable as a
            /// float of the same type. We have
            /// $$
            /// f(x) = (\frac{x}{2^{\lfloor \log_2 x \rfloor}}, \lfloor \log_2 x \rfloor).
            /// $$
            ///
            /// The inverse operation is `from_sci_mantissa_and_exponent`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn sci_mantissa_and_exponent(self) -> ($t, i64) {
                _sci_mantissa_and_exponent_primitive_float(self)
            }

            /// Returns the scientific mantissa.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_s}m_s$, where
            /// $e_s$ is an integer and $m_s$ is a rational number with $1 \leq m_s < 2$. If $x$
            /// is a valid float, the scientific mantissa $m_s$ is always exactly representable as
            /// a float of the same type. We have
            /// $$
            /// f(x) = \frac{x}{2^{\lfloor \log_2 x \rfloor}}.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn sci_mantissa(self) -> $t {
                _sci_mantissa_primitive_float(self)
            }

            /// Returns the scientific exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_s}m_s$, where
            /// $e_s$ is an integer and $m_s$ is a rational number with $1 \leq m_s < 2$. We have
            /// $$
            /// f(x) = \lfloor \log_2 x \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn sci_exponent(self) -> i64 {
                _sci_exponent_primitive_float(self)
            }

            /// Constructs a float from its scientific mantissa and exponent.
            ///
            /// When $x$ is positive, nonzero, and finite, we can write $x = 2^{e_s}m_s$, where
            /// $e_s$ is an integer and $m_s$ is a rational number with $1 \leq m_s < 2$.
            ///
            /// $$
            /// f(x) = 2^{e_s}m_s,
            /// $$
            /// or `None` if the result cannot be exactly represented as a float of the desired
            /// type (this happens if the exponent is too large or too small, if the mantissa is
            /// not in the range $[1, 2)$, or if the mantissa's precision is too high for the
            /// exponent).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `mantissa` is zero, infinite, or `NaN`.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::mantissa_and_exponent` module.
            #[inline]
            fn from_sci_mantissa_and_exponent(sci_mantissa: $t, sci_exponent: i64) -> Option<$t> {
                _from_sci_mantissa_and_exponent_primitive_float(sci_mantissa, sci_exponent)
            }
        }
    };
}
apply_to_primitive_floats!(impl_mantissa_and_exponent_primitive_float);
