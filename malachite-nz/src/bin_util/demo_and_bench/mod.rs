use malachite_base::test_util::runner::Runner;

macro_rules! natural_signed_single_arg_demo_with_trait {
    ($name: ident, $f: ident, $gen: ident, $tr: ident) => {
        fn $name<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize)
        where
            Natural: $tr<T>,
        {
            for x in $gen::<T>().get(gm, &config).take(limit) {
                println!(
                    concat!("Natural::", stringify!($f), "({}) = {}"),
                    x,
                    Natural::$f(x)
                );
            }
        }
    };
}

macro_rules! natural_signed_single_arg_bench_with_trait {
    ($name: ident, $f: ident, $gen: ident, $tr: ident) => {
        fn $name<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize, file_name: &str)
        where
            Natural: $tr<T>,
        {
            run_benchmark(
                &format!(concat!("Natural::", stringify!($f), "({})"), T::NAME),
                BenchmarkType::Single,
                $gen::<T>().get(gm, &config),
                gm.name(),
                limit,
                file_name,
                &signed_bit_bucketer(),
                &mut [("Malachite", &mut |x| no_out!(Natural::$f(x)))],
            );
        }
    };
}

pub(crate) fn register(runner: &mut Runner) {
    integer::register(runner);
    natural::register(runner);
}

mod integer;
mod natural;
