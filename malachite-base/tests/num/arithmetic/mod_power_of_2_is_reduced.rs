use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::unsigned_pair_gen_var_2;

fn mod_power_of_2_is_reduced_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.mod_power_of_2_is_reduced(pow), out);
    };

    test(T::ZERO, 5, true);
    test(T::exact_from(100), 5, false);
    test(T::exact_from(100), 8, true);
    test(T::MAX, T::WIDTH - 1, false);
    test(T::MAX, T::WIDTH, true);
}

#[test]
fn test_mod_power_of_2_is_reduced() {
    apply_fn_to_unsigneds!(mod_power_of_2_is_reduced_helper);
}

fn mod_power_of_2_is_reduced_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(n, pow)| {
        assert_eq!(n.mod_power_of_2_is_reduced(pow), n.mod_power_of_2(pow) == n);
    });
}

#[test]
fn mod_power_of_2_is_reduced_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_is_reduced_properties_helper);
}
