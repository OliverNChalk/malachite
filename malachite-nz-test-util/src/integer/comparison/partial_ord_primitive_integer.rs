use std::cmp::Ordering;

use num::BigInt;

pub fn num_partial_cmp_primitive<T>(x: &BigInt, u: T) -> Option<Ordering>
where
    BigInt: From<T>,
{
    x.partial_cmp(&BigInt::from(u))
}
