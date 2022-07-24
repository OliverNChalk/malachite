use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    add::register(runner);
    add_mul::register(runner);
    checked_sub::register(runner);
    checked_sub_mul::register(runner);
    coprime_with::register(runner);
    div::register(runner);
    div_exact::register(runner);
    div_mod::register(runner);
    div_round::register(runner);
    divisible_by::register(runner);
    divisible_by_power_of_2::register(runner);
    eq_mod::register(runner);
    eq_mod_power_of_2::register(runner);
    extended_gcd::register(runner);
    factorial::register(runner);
    gcd::register(runner);
    is_power_of_2::register(runner);
    kronecker_symbol::register(runner);
    lcm::register(runner);
    log_base::register(runner);
    log_base_2::register(runner);
    log_base_power_of_2::register(runner);
    mod_add::register(runner);
    mod_inverse::register(runner);
    mod_is_reduced::register(runner);
    mod_mul::register(runner);
    mod_neg::register(runner);
    mod_op::register(runner);
    mod_pow::register(runner);
    mod_power_of_2::register(runner);
    mod_power_of_2_add::register(runner);
    mod_power_of_2_inverse::register(runner);
    mod_power_of_2_is_reduced::register(runner);
    mod_power_of_2_mul::register(runner);
    mod_power_of_2_neg::register(runner);
    mod_power_of_2_pow::register(runner);
    mod_power_of_2_shl::register(runner);
    mod_power_of_2_shr::register(runner);
    mod_power_of_2_square::register(runner);
    mod_power_of_2_sub::register(runner);
    mod_shl::register(runner);
    mod_shr::register(runner);
    mod_square::register(runner);
    mod_sub::register(runner);
    mul::register(runner);
    neg::register(runner);
    next_power_of_2::register(runner);
    parity::register(runner);
    pow::register(runner);
    power_of_2::register(runner);
    root::register(runner);
    round_to_multiple::register(runner);
    round_to_multiple_of_power_of_2::register(runner);
    saturating_sub::register(runner);
    saturating_sub_mul::register(runner);
    shl::register(runner);
    shl_round::register(runner);
    shr::register(runner);
    shr_round::register(runner);
    sign::register(runner);
    sqrt::register(runner);
    square::register(runner);
    sub::register(runner);
    sub_mul::register(runner);
}

mod add;
mod add_mul;
mod checked_sub;
mod checked_sub_mul;
mod coprime_with;
mod div;
mod div_exact;
mod div_mod;
mod div_round;
mod divisible_by;
mod divisible_by_power_of_2;
mod eq_mod;
mod eq_mod_power_of_2;
mod extended_gcd;
mod factorial;
mod gcd;
mod is_power_of_2;
mod kronecker_symbol;
mod lcm;
mod log_base;
mod log_base_2;
mod log_base_power_of_2;
mod mod_add;
mod mod_inverse;
mod mod_is_reduced;
mod mod_mul;
mod mod_neg;
mod mod_op;
mod mod_pow;
mod mod_power_of_2;
mod mod_power_of_2_add;
mod mod_power_of_2_inverse;
mod mod_power_of_2_is_reduced;
mod mod_power_of_2_mul;
mod mod_power_of_2_neg;
mod mod_power_of_2_pow;
mod mod_power_of_2_shl;
mod mod_power_of_2_shr;
mod mod_power_of_2_square;
mod mod_power_of_2_sub;
mod mod_shl;
mod mod_shr;
mod mod_square;
mod mod_sub;
mod mul;
mod neg;
mod next_power_of_2;
mod parity;
mod pow;
mod power_of_2;
mod root;
mod round_to_multiple;
mod round_to_multiple_of_power_of_2;
mod saturating_sub;
mod saturating_sub_mul;
mod shl;
mod shl_round;
mod shr;
mod shr_round;
mod sign;
mod sqrt;
mod square;
mod sub;
mod sub_mul;
