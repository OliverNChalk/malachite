use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base_test_util::bench::bucketers::assign_bits_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    signed_unsigned_unsigned_unsigned_quadruple_gen_var_1, unsigned_quadruple_gen_var_1,
};
use malachite_base_test_util::num::logic::bit_block_access::assign_bits_naive;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_assign_bits_unsigned);
    register_signed_unsigned_match_demos!(runner, demo_assign_bits_signed);
    register_unsigned_benches!(runner, benchmark_assign_bits_algorithms_unsigned);
    register_signed_unsigned_match_benches!(runner, benchmark_assign_bits_algorithms_signed);
}

fn demo_assign_bits_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    T::Bits: PrimitiveUnsigned,
{
    for (mut n, start, end, bits) in unsigned_quadruple_gen_var_1::<T, _>()
        .get(gm, &config)
        .take(limit)
    {
        let old_n = n;
        n.assign_bits(start, end, &bits);
        println!(
            "n := {}; n.assign_bits({}, {}, &{}); n = {}",
            old_n, start, end, bits, n,
        );
    }
}

fn demo_assign_bits_signed<
    T: BitBlockAccess<Bits = U> + PrimitiveSigned + UnsignedAbs<Output = U>,
    U: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut n, start, end, bits) in signed_unsigned_unsigned_unsigned_quadruple_gen_var_1::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        let old_n = n;
        n.assign_bits(start, end, &bits);
        println!(
            "n := {}; n.assign_bits({}, {}, &{}); n = {}",
            old_n, start, end, bits, n,
        );
    }
}

fn benchmark_assign_bits_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    T::Bits: PrimitiveUnsigned,
{
    run_benchmark(
        &format!("{}.assign_bits(u64, u64, {})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_quadruple_gen_var_1::<T, _>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &assign_bits_bucketer(),
        &mut [
            ("default", &mut |(mut n, start, end, bits)| {
                no_out!(n.assign_bits(start, end, &bits))
            }),
            ("naive", &mut |(mut n, start, end, bits)| {
                no_out!(assign_bits_naive::<T, T::Bits>(&mut n, start, end, &bits))
            }),
        ],
    );
}

fn benchmark_assign_bits_algorithms_signed<
    T: BitBlockAccess<Bits = U> + PrimitiveSigned + UnsignedAbs<Output = U>,
    U: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.assign_bits(u64, u64, {})", T::NAME, U::NAME),
        BenchmarkType::Algorithms,
        signed_unsigned_unsigned_unsigned_quadruple_gen_var_1::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &assign_bits_bucketer(),
        &mut [
            ("default", &mut |(mut n, start, end, bits)| {
                no_out!(n.assign_bits(start, end, &bits))
            }),
            ("naive", &mut |(mut n, start, end, bits)| {
                no_out!(assign_bits_naive::<T, U>(&mut n, start, end, &bits))
            }),
        ],
    );
}
