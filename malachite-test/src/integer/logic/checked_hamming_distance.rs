use std::cmp::max;
use std::iter::repeat;

use malachite_base::comparison::Max;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::{
    CheckedHammingDistance, HammingDistance, SignificantBits,
};
use malachite_nz::integer::logic::checked_hamming_distance::{
    limbs_hamming_distance_limb_neg, limbs_hamming_distance_neg,
};
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{pairs_of_limb_vec_and_positive_limb_var_1, pairs_of_unsigned_vec_var_6};
use inputs::integer::{pairs_of_integers, rm_pairs_of_integers};

pub fn integer_checked_hamming_distance_alt_1(x: &Integer, y: &Integer) -> Option<u64> {
    let negative = *x < 0 as Limb;
    if negative != (*y < 0 as Limb) {
        return None;
    }
    let bit_zip: Box<dyn Iterator<Item = (bool, bool)>> =
        if x.twos_complement_bits().count() >= y.twos_complement_bits().count() {
            Box::new(
                x.twos_complement_bits()
                    .zip(y.twos_complement_bits().chain(repeat(negative))),
            )
        } else {
            Box::new(
                x.twos_complement_bits()
                    .chain(repeat(negative))
                    .zip(y.twos_complement_bits()),
            )
        };
    let mut distance = 0u64;
    for (b, c) in bit_zip {
        if b != c {
            distance += 1;
        }
    }
    Some(distance)
}

pub fn rug_checked_hamming_distance(x: &rug::Integer, y: &rug::Integer) -> Option<u64> {
    x.hamming_dist(y).map(|u| u64::from(u))
}

pub fn integer_checked_hamming_distance_alt_2(x: &Integer, y: &Integer) -> Option<u64> {
    if (*x < 0 as Limb) != (*y < 0 as Limb) {
        return None;
    }
    let extension = if *x < 0 as Limb { Limb::MAX } else { 0 };
    let limb_zip: Box<dyn Iterator<Item = (Limb, Limb)>> =
        if x.twos_complement_limbs().count() >= y.twos_complement_limbs().count() {
            Box::new(
                x.twos_complement_limbs()
                    .zip(y.twos_complement_limbs().chain(repeat(extension))),
            )
        } else {
            Box::new(
                x.twos_complement_limbs()
                    .chain(repeat(extension))
                    .zip(y.twos_complement_limbs()),
            )
        };
    let mut distance = 0u64;
    for (x, y) in limb_zip {
        distance += x.hamming_distance(y);
    }
    Some(distance)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_hamming_distance_limb_neg);
    register_demo!(registry, demo_limbs_hamming_distance_neg);
    register_demo!(registry, demo_integer_checked_hamming_distance);
    register_bench!(registry, Small, benchmark_limbs_hamming_distance_limb_neg);
    register_bench!(registry, Small, benchmark_limbs_hamming_distance_neg);
    register_bench!(
        registry,
        Large,
        benchmark_integer_checked_hamming_distance_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_checked_hamming_distance_algorithms
    );
}

fn demo_limbs_hamming_distance_limb_neg(gm: GenerationMode, limit: usize) {
    for (ref limbs, limb) in pairs_of_limb_vec_and_positive_limb_var_1(gm).take(limit) {
        println!(
            "limbs_hamming_distance_limb_neg({:?}, {}) = {}",
            limbs,
            limb,
            limbs_hamming_distance_limb_neg(limbs, limb)
        );
    }
}

fn demo_limbs_hamming_distance_neg(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_unsigned_vec_var_6(gm).take(limit) {
        println!(
            "limbs_hamming_distance_neg({:?}, {:?}) = {}",
            xs,
            ys,
            limbs_hamming_distance_neg(xs, ys)
        );
    }
}

fn demo_integer_checked_hamming_distance(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            x,
            y,
            x.checked_hamming_distance(&y)
        );
    }
}

fn benchmark_limbs_hamming_distance_limb_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_hamming_distance_limb_neg(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_limb_vec_and_positive_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(ref limbs, limb)| no_out!(limbs_hamming_distance_limb_neg(limbs, limb))),
        )],
    );
}

fn benchmark_limbs_hamming_distance_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_hamming_distance_neg(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(ref xs, ref ys)| no_out!(limbs_hamming_distance_neg(xs, ys))),
        )],
    );
}

fn benchmark_integer_checked_hamming_distance_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.checked_hamming_distance(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "malachite",
                &mut (|(_, (x, y))| no_out!(x.checked_hamming_distance(&y))),
            ),
            (
                "rug",
                &mut (|((x, y), _)| no_out!(rug_checked_hamming_distance(&x, &y))),
            ),
        ],
    );
}

fn benchmark_integer_checked_hamming_distance_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.checked_hamming_distance(&Integer)",
        BenchmarkType::Algorithms,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "default",
                &mut (|(n, other)| no_out!(n.checked_hamming_distance(&other))),
            ),
            (
                "using bits explicitly",
                &mut (|(n, other)| no_out!(integer_checked_hamming_distance_alt_1(&n, &other))),
            ),
            (
                "using limbs explicitly",
                &mut (|(n, other)| no_out!(integer_checked_hamming_distance_alt_2(&n, &other))),
            ),
        ],
    );
}
