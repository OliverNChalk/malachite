use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint};
use malachite_native::integer as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, BenchmarkOptions4, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_partial_eq_i32(x: &num::BigInt, i: i32) -> bool {
    *x == num::BigInt::from(i)
}

pub fn demo_exhaustive_integer_partial_eq_i32(limit: usize) {
    for (n, i) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(limit) {
        if n == i {
            println!("{} = {}", n, i);
        } else {
            println!("{} ≠ {}", n, i);
        }
    }
}

pub fn demo_random_integer_partial_eq_i32(limit: usize) {
    for (n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
    ).take(limit)
    {
        if n == i {
            println!("{} = {}", n, i);
        } else {
            println!("{} ≠ {}", n, i);
        }
    }
}

pub fn demo_exhaustive_i32_partial_eq_integer(limit: usize) {
    for (i, n) in exhaustive_pairs(exhaustive_i::<i32>(), exhaustive_integers()).take(limit) {
        if i == n {
            println!("{} = {}", i, n);
        } else {
            println!("{} ≠ {}", i, n);
        }
    }
}

pub fn demo_random_i32_partial_eq_integer(limit: usize) {
    for (i, n) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x::<i32>(seed)),
        &(|seed| random_integers(seed, 32)),
    ).take(limit)
    {
        if i == n {
            println!("{} = {}", i, n);
        } else {
            println!("{} ≠ {}", i, n);
        }
    }
}

pub fn benchmark_exhaustive_integer_partial_eq_i32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer == i32");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        function_f: &(|(n, i)| n == i),
        function_g: &(|(n, i): (native::Integer, i32)| n == i),
        function_h: &(|(n, i): (num::BigInt, i32)| num_partial_eq_i32(&n, i)),
        function_i: &(|(n, i): (rugint::Integer, i32)| n == i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        z_cons: &(|&(ref n, i)| (gmp_integer_to_num_bigint(n), i)),
        w_cons: &(|&(ref n, i)| (gmp_integer_to_rugint(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer == i32",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_partial_eq_i32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer == i32");
    benchmark_4(BenchmarkOptions4 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<i32>(seed)),
        ),
        function_f: &(|(n, i)| n == i),
        function_g: &(|(n, i): (native::Integer, i32)| n == i),
        function_h: &(|(n, i): (num::BigInt, i32)| num_partial_eq_i32(&n, i)),
        function_i: &(|(n, i): (rugint::Integer, i32)| n == i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        z_cons: &(|&(ref n, i)| (gmp_integer_to_num_bigint(n), i)),
        w_cons: &(|&(ref n, i)| (gmp_integer_to_rugint(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer == i32",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_i32_partial_eq_integer(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive i32 == Integer");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_i::<i32>(), exhaustive_integers()),
        function_f: &(|(i, n)| i == n),
        function_g: &(|(i, n): (i32, native::Integer)| i == n),
        function_h: &(|(i, n): (i32, rugint::Integer)| i == n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(i, ref n)| (i, gmp_integer_to_native(n))),
        z_cons: &(|&(i, ref n)| (i, gmp_integer_to_rugint(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "i32 == Integer",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_i32_partial_eq_integer(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random i32 == Integer");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_x::<i32>(seed)),
            &(|seed| random_integers(seed, scale)),
        ),
        function_f: &(|(i, n)| i == n),
        function_g: &(|(i, n): (i32, native::Integer)| i == n),
        function_h: &(|(i, n): (i32, rugint::Integer)| i == n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(i, ref n)| (i, gmp_integer_to_native(n))),
        z_cons: &(|&(i, ref n)| (i, gmp_integer_to_rugint(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "i32 == Integer",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
