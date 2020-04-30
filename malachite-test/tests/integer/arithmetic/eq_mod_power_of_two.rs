use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOfTwo, EqModPowerOfTwo, ModPowerOfTwo,
};
use malachite_base::num::basic::traits::Zero;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::arithmetic::eq_mod_power_of_two::{
    limbs_eq_mod_power_of_two_neg_limb, limbs_eq_mod_power_of_two_neg_pos,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;
use malachite_nz::platform::SignedLimb;

#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::integer_to_rug_integer;
use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    triples_of_signed_signed_and_small_unsigned,
    triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2,
    triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_small_unsigned, pairs_of_integers,
    quadruples_of_integer_integer_integer_and_small_unsigned,
    triples_of_integer_integer_and_small_unsigned,
    triples_of_integer_integer_and_small_unsigned_var_1,
    triples_of_integer_integer_and_small_unsigned_var_2,
};
use malachite_test::inputs::natural::triples_of_natural_natural_and_small_unsigned;

#[test]
fn limbs_eq_mod_power_of_two_neg_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2,
        |&(ref limbs, limb, pow)| {
            assert_eq!(
                limbs_eq_mod_power_of_two_neg_limb(limbs, limb, pow),
                (-Natural::from_limbs_asc(limbs)).eq_mod_power_of_two(&Integer::from(limb), pow)
            );
        },
    );
}

#[test]
fn limbs_eq_mod_power_of_two_neg_pos_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1,
        |&(ref xs, ref ys, pow)| {
            assert_eq!(
                limbs_eq_mod_power_of_two_neg_pos(xs, ys, pow),
                (-Natural::from_limbs_asc(xs))
                    .eq_mod_power_of_two(&Integer::from(Natural::from_limbs_asc(ys)), pow)
            );
        },
    );
}

#[test]
fn eq_mod_power_of_two_properties() {
    test_properties(
        triples_of_integer_integer_and_small_unsigned,
        |&(ref x, ref y, pow)| {
            let eq_mod_power_of_two = x.eq_mod_power_of_two(y, pow);
            #[cfg(feature = "32_bit_limbs")]
            assert_eq!(
                integer_to_rug_integer(x)
                    .is_congruent_2pow(&integer_to_rug_integer(y), Limb::exact_from(pow),),
                eq_mod_power_of_two
            );
            assert_eq!(y.eq_mod_power_of_two(x, pow), eq_mod_power_of_two);
            assert_eq!(
                x.mod_power_of_two(pow) == y.mod_power_of_two(pow),
                eq_mod_power_of_two,
            );
        },
    );

    test_properties(
        triples_of_integer_integer_and_small_unsigned_var_1::<u64>,
        |&(ref x, ref y, pow)| {
            assert!(x.eq_mod_power_of_two(y, pow));
            #[cfg(feature = "32_bit_limbs")]
            assert!(integer_to_rug_integer(x)
                .is_congruent_2pow(&integer_to_rug_integer(y), Limb::exact_from(pow)));
            assert!(y.eq_mod_power_of_two(x, pow));
            assert_eq!(x.mod_power_of_two(pow), y.mod_power_of_two(pow));
        },
    );

    test_properties(
        triples_of_integer_integer_and_small_unsigned_var_2::<u64>,
        |&(ref x, ref y, pow)| {
            assert!(!x.eq_mod_power_of_two(y, pow));
            #[cfg(feature = "32_bit_limbs")]
            assert!(!integer_to_rug_integer(x)
                .is_congruent_2pow(&integer_to_rug_integer(y), Limb::exact_from(pow)));
            assert!(!y.eq_mod_power_of_two(x, pow));
            assert_ne!(x.mod_power_of_two(pow), y.mod_power_of_two(pow));
        },
    );

    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, pow)| {
        assert!(n.eq_mod_power_of_two(n, pow));
        assert_eq!(
            n.eq_mod_power_of_two(&Integer::ZERO, pow),
            n.divisible_by_power_of_two(pow)
        );
        assert_eq!(
            Integer::ZERO.eq_mod_power_of_two(n, pow),
            n.divisible_by_power_of_two(pow)
        );
    });

    test_properties(
        quadruples_of_integer_integer_integer_and_small_unsigned,
        |&(ref x, ref y, ref z, pow)| {
            if x.eq_mod_power_of_two(y, pow) && y.eq_mod_power_of_two(z, pow) {
                assert!(x.eq_mod_power_of_two(z, pow));
            }
        },
    );

    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        assert!(x.eq_mod_power_of_two(y, 0));
    });

    test_properties(
        triples_of_natural_natural_and_small_unsigned,
        |&(ref x, ref y, pow)| {
            assert_eq!(
                x.eq_mod_power_of_two(y, pow),
                Integer::from(x).eq_mod_power_of_two(&Integer::from(y), pow),
            );
        },
    );

    test_properties(
        triples_of_signed_signed_and_small_unsigned::<SignedLimb, u64>,
        |&(x, y, pow)| {
            assert_eq!(
                x.eq_mod_power_of_two(y, pow),
                Integer::from(x).eq_mod_power_of_two(&Integer::from(y), pow),
            );
        },
    );
}
