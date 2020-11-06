#![allow(
    unstable_name_collisions,
    clippy::assertions_on_constants,
    clippy::cognitive_complexity,
    clippy::float_cmp,
    clippy::many_single_char_names,
    clippy::range_plus_one,
    clippy::suspicious_arithmetic_impl,
    clippy::suspicious_op_assign_impl,
    clippy::too_many_arguments
)]
#![warn(
    clippy::cast_lossless,
    clippy::decimal_literal_representation,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map,
    clippy::filter_map_next,
    clippy::find_map,
    clippy::large_digit_groups,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::missing_const_for_fn,
    clippy::mut_mut,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::print_stdout,
    clippy::redundant_closure_for_method_calls,
    clippy::single_match_else,
    clippy::trait_duplication_in_bounds,
    clippy::type_repetition_in_bounds,
    clippy::unused_self
)]

extern crate clap;
extern crate gnuplot;
extern crate itertools;
#[macro_use]
extern crate malachite_base;
extern crate ryu;
extern crate time;

pub mod bench;
pub mod common;
pub mod generators;
pub mod hash;
pub mod num {
    pub mod arithmetic {
        pub mod mod_mul;
        pub mod mod_pow;
    }
    pub mod float {
        pub mod nice_float;
    }
    pub mod logic {
        pub mod bit_block_access;
        pub mod bit_convertible;
    }
    pub mod random {
        pub mod geometric;
    }
}
pub mod rounding_modes;
pub mod runner;
pub mod slices;
pub mod stats;
