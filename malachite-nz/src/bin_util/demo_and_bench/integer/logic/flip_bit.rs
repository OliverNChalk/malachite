use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_2_pair_integer_bit_u64_max_bucketer;
use malachite_nz::test_util::generators::{
    integer_unsigned_pair_gen_var_2, integer_unsigned_pair_gen_var_2_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_flip_bit);
    register_bench!(runner, benchmark_integer_flip_bit_library_comparison);
}

fn demo_integer_flip_bit(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, index) in integer_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n.clone();
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_integer_flip_bit_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.flip_bit(u64)",
        BenchmarkType::LibraryComparison,
        integer_unsigned_pair_gen_var_2_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_integer_bit_u64_max_bucketer("x", "index"),
        &mut [
            ("Malachite", &mut |(_, (mut n, index))| n.flip_bit(index)),
            ("rug", &mut |((mut n, index), _)| {
                no_out!(n.toggle_bit(u32::exact_from(index)))
            }),
        ],
    );
}
