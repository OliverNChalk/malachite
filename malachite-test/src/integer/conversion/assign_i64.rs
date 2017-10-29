use common::{gmp_integer_to_native, gmp_integer_to_num_bigint};
use malachite_base::traits::Assign;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_assign_i64(x: &mut num::BigInt, i: i64) {
    *x = num::BigInt::from(i);
}

pub fn demo_exhaustive_integer_assign_i64(limit: usize) {
    for (mut n, i) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i64>()).take(limit) {
        let n_old = n.clone();
        n.assign(i);
        println!("x := {}; x.assign({}); x = {}", n_old, i, n);
    }
}

pub fn demo_random_integer_assign_i64(limit: usize) {
    for (mut n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i64>(seed)),
    ).take(limit)
    {
        let n_old = n.clone();
        n.assign(i);
        println!("x := {}; x.assign({}); x = {}", n_old, i, n);
    }
}

pub fn benchmark_exhaustive_integer_assign_i64(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.assign(i64)");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i64>()),
        function_f: &(|(mut n, i): (gmp::Integer, i64)| n.assign(i)),
        function_g: &(|(mut n, i): (native::Integer, i64)| n.assign(i)),
        function_h: &(|(mut n, i): (num::BigInt, i64)| num_assign_i64(&mut n, i)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        z_cons: &(|&(ref n, i)| (gmp_integer_to_num_bigint(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        title: "Integer.assign(i64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_assign_i64(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.assign(i64)");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<i64>(seed)),
        ),
        function_f: &(|(mut n, i): (gmp::Integer, i64)| n.assign(i)),
        function_g: &(|(mut n, i): (native::Integer, i64)| n.assign(i)),
        function_h: &(|(mut n, i): (num::BigInt, i64)| num_assign_i64(&mut n, i)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        z_cons: &(|&(ref n, i)| (gmp_integer_to_num_bigint(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        title: "Integer.assign(i64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
