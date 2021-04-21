use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::pair_2_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_pair_gen_var_17;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_power_of_2_neg);
    register_unsigned_demos!(runner, demo_mod_power_of_2_neg_assign);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_neg);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_neg_assign);
}

fn demo_mod_power_of_2_neg<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, pow) in unsigned_pair_gen_var_17::<T>().get(gm, &config).take(limit) {
        println!("-{} === {} mod 2^{}", n, n.mod_power_of_2_neg(pow), pow);
    }
}

fn demo_mod_power_of_2_neg_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut n, pow) in unsigned_pair_gen_var_17::<T>().get(gm, &config).take(limit) {
        let old_n = n;
        n.mod_power_of_2_neg_assign(pow);
        println!(
            "n := {}; n.mod_power_of_2_neg_assign({}); n = {}",
            old_n, pow, n
        );
    }
}

fn benchmark_mod_power_of_2_neg<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_neg(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_17::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(n, pow)| {
            no_out!(n.mod_power_of_2_neg(pow))
        })],
    );
}

fn benchmark_mod_power_of_2_neg_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_neg_assign(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_17::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(mut n, pow)| {
            n.mod_power_of_2_neg_assign(pow)
        })],
    );
}
