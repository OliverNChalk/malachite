use std::cmp::max;
use std::str::FromStr;

use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::logic::xor::{
    limbs_xor, limbs_xor_in_place_either, limbs_xor_in_place_left, limbs_xor_limb,
    limbs_xor_limb_in_place, limbs_xor_limb_to_out, limbs_xor_same_length,
    limbs_xor_same_length_in_place_left, limbs_xor_same_length_to_out, limbs_xor_to_out,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_unsigned_vec,
    pairs_of_unsigned_vec_var_1, pairs_of_unsigneds,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2, triples_of_unsigned_vec_var_3,
    triples_of_unsigned_vec_var_4,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};
use malachite_test::natural::logic::xor::{natural_xor_alt_1, natural_xor_alt_2};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_limb_and_limbs_xor_limb_in_place() {
    let test = |limbs: &[Limb], limb: Limb, out: &[Limb]| {
        assert_eq!(limbs_xor_limb(limbs, limb), out);

        let mut limbs = limbs.to_vec();
        limbs_xor_limb_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[6, 7], 2, &[4, 7]);
    test(&[100, 101, 102], 10, &[110, 101, 102]);
    test(&[123, 456], 789, &[878, 456]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_limb_fail() {
    limbs_xor_limb(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_limb_in_place_fail() {
    limbs_xor_limb_in_place(&mut [], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_limb_to_out() {
    let test = |out_before: &[Limb], limbs_in: &[Limb], limb: Limb, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_xor_limb_to_out(&mut out, limbs_in, limb);
        assert_eq!(out, out_after);
    };
    test(&[10, 10, 10, 10], &[6, 7], 2, &[4, 7, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        &[110, 101, 102, 10],
    );
    test(&[10, 10, 10, 10], &[123, 456], 789, &[878, 456, 10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_limb_to_out_fail_1() {
    limbs_xor_limb_to_out(&mut [], &[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_limb_to_out_fail_2() {
    limbs_xor_limb_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_same_length_and_limbs_xor_same_length_in_place_left() {
    let test = |xs_before, ys, out| {
        assert_eq!(limbs_xor_same_length(xs_before, ys), out);

        let mut xs = xs_before.to_vec();
        limbs_xor_same_length_in_place_left(&mut xs, ys);
        assert_eq!(xs, out);
    };
    test(&[], &[], vec![]);
    test(&[2], &[3], vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], vec![0, 3, 2]);
    test(&[6, 7], &[1, 2], vec![7, 5]);
    test(&[100, 101, 102], &[102, 101, 100], vec![2, 0, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_same_length_fail_1() {
    limbs_xor_same_length(&[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_same_length_in_place_left_fail() {
    let mut out = vec![6, 7];
    limbs_xor_same_length_in_place_left(&mut out, &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_and_limbs_xor_in_place_left() {
    let test = |xs_before, ys, out| {
        assert_eq!(limbs_xor(xs_before, ys), out);

        let mut xs = xs_before.to_vec();
        limbs_xor_in_place_left(&mut xs, ys);
        assert_eq!(xs, out);
    };
    test(&[], &[], vec![]);
    test(&[2], &[3], vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], vec![0, 3, 2]);
    test(&[6, 7], &[1, 2, 3], vec![7, 5, 3]);
    test(&[1, 2, 3], &[6, 7], vec![7, 5, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![2, 0, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_same_length_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_xor_same_length_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], vec![0, 0]);
    test(&[1, 1, 1], &[1, 2, 3], &[5, 5, 5, 5], vec![0, 3, 2, 5]);
    test(&[6, 7], &[1, 2], &[0, 0], vec![7, 5]);
    test(&[6, 7], &[1, 2], &[10, 10, 10, 10], vec![7, 5, 10, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![2, 0, 2, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_same_length_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_xor_same_length_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_same_length_to_out_fail_2() {
    let mut out = vec![10];
    limbs_xor_same_length_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_xor_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], vec![0, 0]);
    test(&[1, 1, 1], &[1, 2, 3], &[5, 5, 5, 5], vec![0, 3, 2, 5]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![7, 5, 3, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![7, 5, 3, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![2, 0, 2, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_to_out_fail() {
    let mut out = vec![10, 10];
    limbs_xor_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_in_place_either() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], right, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_xor_in_place_either(&mut xs, &mut ys), right);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], false, vec![], vec![]);
    test(&[6, 7], &[1, 2], false, vec![7, 5], vec![1, 2]);
    test(&[6, 7], &[1, 2, 3], true, vec![6, 7], vec![7, 5, 3]);
    test(&[1, 2, 3], &[6, 7], false, vec![7, 5, 3], vec![6, 7]);
    test(&[], &[1, 2, 3], true, vec![], vec![1, 2, 3]);
    test(&[1, 2, 3], &[], false, vec![1, 2, 3], vec![]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![0, 3, 2], vec![1, 2, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![2, 0, 2],
        vec![102, 101, 100],
    );
}

#[test]
fn test_xor() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n ^= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n ^= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() ^ Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() ^ Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() ^ &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() ^ &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            natural_xor_alt_1(
                &Natural::from_str(u).unwrap(),
                &Natural::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );
        assert_eq!(
            natural_xor_alt_2(
                &Natural::from_str(u).unwrap(),
                &Natural::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );

        let n = BigUint::from_str(u).unwrap() ^ BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() ^ rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "435");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("1000000000001", "123", "1000000000122");
    test("12345678987654321", "0", "12345678987654321");
    test("12345678987654321", "456", "12345678987654521");
    test("12345678987654321", "987654321", "12345678815534080");
    test("1000000000000", "999999999999", "8191");
    test("12345678987654321", "314159265358979", "12035174921130034");
}

#[test]
fn limbs_xor_limb_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_xor_limb(limbs, limb)),
                Natural::from_limbs_asc(limbs) ^ Natural::from(limb)
            );
        },
    );
}

#[test]
fn limbs_xor_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            limbs_xor_limb_to_out(&mut out, in_limbs, limb);
            let len = in_limbs.len();
            assert_eq!(
                Natural::from_limbs_asc(&out[..len]),
                Natural::from_limbs_asc(in_limbs) ^ Natural::from(limb)
            );
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_xor_limb_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_xor_limb_in_place(&mut limbs, limb);
            assert_eq!(
                Natural::from_limbs_asc(&limbs),
                Natural::from_limbs_asc(&old_limbs) ^ Natural::from(limb)
            );
        },
    );
}

fn limbs_xor_helper(f: &dyn Fn(&[Limb], &[Limb]) -> Vec<Limb>, xs: &Vec<Limb>, ys: &Vec<Limb>) {
    assert_eq!(
        Natural::from_owned_limbs_asc(f(xs, ys)),
        Natural::from_limbs_asc(xs) ^ Natural::from_limbs_asc(ys)
    );
}

#[test]
fn limbs_xor_same_length_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        limbs_xor_helper(&mut limbs_xor_same_length, xs, ys);
    });
}

#[test]
fn limbs_xor_properties() {
    test_properties(pairs_of_unsigned_vec, |&(ref xs, ref ys)| {
        limbs_xor_helper(&mut limbs_xor, xs, ys);
    });
}

#[test]
fn limbs_xor_same_length_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_3,
        |&(ref xs, ref ys, ref zs)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            limbs_xor_same_length_to_out(&mut xs, ys, zs);
            let len = ys.len();
            assert_eq!(
                Natural::from_limbs_asc(&xs[..len]),
                Natural::from_limbs_asc(ys) ^ Natural::from_limbs_asc(zs)
            );
            assert_eq!(&xs[len..], &xs_old[len..]);
        },
    );
}

#[test]
fn limbs_xor_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_4,
        |&(ref xs, ref ys, ref zs)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            limbs_xor_to_out(&mut xs, ys, zs);
            let len = max(ys.len(), zs.len());
            assert_eq!(
                Natural::from_limbs_asc(&xs[..len]),
                Natural::from_limbs_asc(ys) ^ Natural::from_limbs_asc(zs)
            );
            assert_eq!(&xs[len..], &xs_old[len..]);
        },
    );
}

macro_rules! limbs_xor_in_place_left_helper {
    ($f:ident, $xs:ident, $ys:ident) => {
        |&(ref $xs, ref $ys)| {
            let mut xs = $xs.to_vec();
            let xs_old = xs.clone();
            $f(&mut xs, $ys);
            assert_eq!(
                Natural::from_owned_limbs_asc(xs),
                Natural::from_owned_limbs_asc(xs_old) ^ Natural::from_limbs_asc($ys)
            );
        }
    };
}

#[test]
fn limbs_xor_same_length_in_place_left_properties() {
    test_properties(
        pairs_of_unsigned_vec_var_1,
        limbs_xor_in_place_left_helper!(limbs_xor_same_length_in_place_left, xs, ys),
    );
}

#[test]
fn limbs_xor_in_place_left_properties() {
    test_properties(
        pairs_of_unsigned_vec,
        limbs_xor_in_place_left_helper!(limbs_xor_in_place_left, xs, ys),
    );
}

#[test]
fn limbs_xor_in_place_either_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let mut ys = ys.to_vec();
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_xor_in_place_either(&mut xs, &mut ys);
        let n = Natural::from_limbs_asc(&xs_old) ^ Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(Natural::from_owned_limbs_asc(ys), n);
        } else {
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn xor_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let result_val_val = x.clone() ^ y.clone();
        let result_val_ref = x.clone() ^ y;
        let result_ref_val = x ^ y.clone();
        let result = x ^ y;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result.is_valid());
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let mut mut_x = x.clone();
        mut_x ^= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, result);
        let mut mut_x = x.clone();
        mut_x ^= y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_x = natural_to_rug_integer(x);
        mut_x ^= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&mut_x), result);

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) ^ natural_to_biguint(y))),
            result
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) ^ natural_to_rug_integer(y))),
            result
        );

        assert_eq!(natural_xor_alt_1(&x, y), result);
        assert_eq!(natural_xor_alt_2(&x, y), result);

        assert_eq!(y ^ x, result);
        assert_eq!(&result ^ x, *y);
        assert_eq!(&result ^ y, *x);
    });

    test_properties(naturals, |x| {
        assert_eq!(x ^ Natural::ZERO, *x);
        assert_eq!(Natural::ZERO ^ x, *x);
        assert_eq!(x ^ x, 0);
    });

    test_properties(triples_of_naturals, |&(ref x, ref y, ref z)| {
        assert_eq!((x ^ y) ^ z, x ^ (y ^ z));
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(Natural::from(x) ^ Natural::from(y), x ^ y);
    });
}
