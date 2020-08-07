use std::panic::catch_unwind;

use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_overflowing_div() {
    fn test<T: PrimitiveInteger>(x: T, y: T, out: T, overflow: bool) {
        assert_eq!(x.overflowing_div(y), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_div_assign(y), overflow);
        assert_eq!(x, out);
    };
    test::<u16>(0, 5, 0, false);
    test::<u16>(123, 456, 0, false);
    test::<u8>(100, 3, 33, false);
    test::<i8>(100, -3, -33, false);
    test::<i16>(-100, 3, -33, false);
    test::<i32>(-100, -3, 33, false);
    test::<i8>(-128, -1, -128, true);
}

fn overflowing_div_assign_fail_helper<T: PrimitiveInteger>() {
    assert_panic!(T::ONE.overflowing_div_assign(T::ZERO));
}

#[test]
fn overflowing_div_assign_fail() {
    apply_fn_to_primitive_ints!(overflowing_div_assign_fail_helper);
}
