use std::panic::catch_unwind;

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::exhaustive::exhaustive_signed_range;

fn expected_range_len<T: PrimitiveSigned>(a: T, b: T) -> usize
where
    usize: WrappingFrom<T>,
{
    usize::wrapping_from(b).wrapping_sub(usize::wrapping_from(a))
}

fn exhaustive_signed_range_helper_helper<T: PrimitiveSigned>(a: T, b: T, values: &[i8])
where
    i8: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    let xs = exhaustive_signed_range::<T>(a, b)
        .map(i8::exact_from)
        .take(20)
        .collect::<Vec<_>>();
    assert_eq!(xs, values);
    if T::WIDTH <= u16::WIDTH {
        assert_eq!(
            exhaustive_signed_range(a, b).count(),
            expected_range_len(a, b)
        );
    }
}

fn exhaustive_signed_range_rev_helper<T: PrimitiveSigned>(a: T, b: T, rev_values: &[T])
where
    usize: WrappingFrom<T>,
{
    let len = expected_range_len(a, b);
    assert_eq!(exhaustive_signed_range(a, b).count(), len);
    let mut tail = exhaustive_signed_range::<T>(a, b)
        .skip(len.saturating_sub(20))
        .collect::<Vec<_>>();
    tail.reverse();
    assert_eq!(tail, rev_values);
}

fn exhaustive_signed_range_helper<T: PrimitiveSigned>()
where
    i8: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    exhaustive_signed_range_helper_helper(T::ZERO, T::ZERO, &[]);
    exhaustive_signed_range_helper_helper(T::ZERO, T::ONE, &[0]);
    exhaustive_signed_range_helper_helper(
        T::ZERO,
        T::exact_from(10),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    );
    exhaustive_signed_range_helper_helper(
        T::exact_from(10),
        T::exact_from(20),
        &[10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    );
    exhaustive_signed_range_helper_helper(
        T::ZERO,
        T::MAX,
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
    );
    exhaustive_signed_range_helper_helper(
        T::ZERO,
        T::MAX - T::ONE,
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
    );
    exhaustive_signed_range_helper_helper(
        T::exact_from(-20),
        T::exact_from(-10),
        &[-11, -12, -13, -14, -15, -16, -17, -18, -19, -20],
    );
    exhaustive_signed_range_helper_helper(
        T::exact_from(-100),
        T::exact_from(100),
        &[
            0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10,
        ],
    );
    exhaustive_signed_range_helper_helper(
        T::MIN,
        T::MAX,
        &[
            0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10,
        ],
    );
    exhaustive_signed_range_helper_helper(
        T::MIN + T::ONE,
        T::MAX - T::ONE,
        &[
            0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10,
        ],
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_exhaustive_signed_range() {
    apply_fn_to_signeds!(exhaustive_signed_range_helper);

    exhaustive_signed_range_rev_helper::<i8>(
        i8::MIN,
        i8::MAX,
        &[
            -128, -127, -126, 126, -125, 125, -124, 124, -123, 123, -122, 122, -121, 121, -120,
            120, -119, 119, -118, 118,
        ],
    );
    exhaustive_signed_range_rev_helper::<i8>(
        i8::MIN + 1,
        i8::MAX - 1,
        &[
            -127, -126, -125, 125, -124, 124, -123, 123, -122, 122, -121, 121, -120, 120, -119,
            119, -118, 118, -117, 117,
        ],
    );
    exhaustive_signed_range_rev_helper::<i16>(
        i16::MIN,
        i16::MAX,
        &[
            -32768, -32767, -32766, 32766, -32765, 32765, -32764, 32764, -32763, 32763, -32762,
            32762, -32761, 32761, -32760, 32760, -32759, 32759, -32758, 32758,
        ],
    );
    exhaustive_signed_range_rev_helper::<i16>(
        i16::MIN + 1,
        i16::MAX - 1,
        &[
            -32767, -32766, -32765, 32765, -32764, 32764, -32763, 32763, -32762, 32762, -32761,
            32761, -32760, 32760, -32759, 32759, -32758, 32758, -32757, 32757,
        ],
    );
}

fn exhaustive_signed_range_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(exhaustive_signed_range::<T>(T::ONE, T::ZERO));
}

#[test]
fn exhaustive_signed_range_fail() {
    apply_fn_to_signeds!(exhaustive_signed_range_fail_helper);
}
