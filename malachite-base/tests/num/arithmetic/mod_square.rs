use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_gen_var_1, unsigned_pair_gen_var_16, unsigned_triple_gen_var_12,
};

fn mod_square_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, m, out| {
        assert_eq!(x.mod_mul(x, m), out);
        assert_eq!(x.mod_square(m), out);

        let mut mut_x = x;
        mut_x.mod_square_assign(m);
        assert_eq!(mut_x, out);

        let data = T::precompute_mod_pow_data(&m);
        assert_eq!(x.mod_square_precomputed(m, &data), out);

        let mut mut_x = x;
        mut_x.mod_square_precomputed_assign(m, &data);
        assert_eq!(mut_x, out);
    };
    test(T::ZERO, T::ONE, T::ZERO);
    test(T::ONE, T::exact_from(10), T::ONE);
    test(T::TWO, T::exact_from(10), T::exact_from(4));
    if T::WIDTH > u8::WIDTH {
        test(T::exact_from(100), T::exact_from(497), T::exact_from(60));
        test(T::exact_from(200), T::exact_from(497), T::exact_from(240));
        test(T::exact_from(300), T::exact_from(497), T::exact_from(43));
    }
}

#[test]
fn test_mod_square() {
    apply_fn_to_unsigneds!(mod_square_helper);
}

fn mod_square_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_16::<T>().test_properties(|(x, m)| {
        assert!(x.mod_is_reduced(&m));
        let square = x.mod_square(m);
        assert!(square.mod_is_reduced(&m));

        let mut x_alt = x;
        x_alt.mod_square_assign(m);
        assert_eq!(x_alt, square);

        let data = T::precompute_mod_pow_data(&m);

        assert_eq!(x.mod_square_precomputed(m, &data), square);

        let mut x_alt = x;
        x_alt.mod_square_precomputed_assign(m, &data);
        assert_eq!(x_alt, square);

        assert_eq!(x.mod_mul(x, m), square);
        assert_eq!(x.mod_neg(m).mod_square(m), square);
    });

    unsigned_gen_var_1::<T>().test_properties(|m| {
        assert_eq!(T::ZERO.mod_square(m), T::ZERO);
        if m != T::ONE {
            assert_eq!(T::ONE.mod_square(m), T::ONE);
        }
    });

    unsigned_triple_gen_var_12::<T>().test_properties(|(x, y, m)| {
        assert_eq!(
            x.mod_mul(y, m).mod_square(m),
            x.mod_square(m).mod_mul(y.mod_square(m), m)
        );
    });
}

#[test]
fn mod_square_properties() {
    apply_fn_to_unsigneds!(mod_square_properties_helper);
}
