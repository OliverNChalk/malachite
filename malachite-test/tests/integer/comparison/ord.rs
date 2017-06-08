use common::test_cmp_helper;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num;
use rugint;

#[test]
fn test_ord() {
    let strings = vec!["-1000000000001",
                       "-1000000000000",
                       "-999999999999",
                       "-123",
                       "-2",
                       "-1",
                       "0",
                       "1",
                       "2",
                       "123",
                       "999999999999",
                       "1000000000000",
                       "1000000000001"];
    test_cmp_helper::<native::Integer>(&strings);
    test_cmp_helper::<gmp::Integer>(&strings);
    test_cmp_helper::<num::BigInt>(&strings);
    test_cmp_helper::<rugint::Integer>(&strings);
}
