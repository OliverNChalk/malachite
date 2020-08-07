use std::panic::catch_unwind;

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_mod_op_unsigned() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, r: T) {
        assert_eq!(n.mod_op(d), r);

        let mut mut_n = n;
        mut_n.mod_assign(d);
        assert_eq!(mut_n, r);
    };
    test::<u8>(0, 1, 0);
    test::<u16>(0, 123, 0);
    test::<u32>(1, 1, 0);
    test::<u64>(123, 1, 0);
    test::<usize>(123, 123, 0);
    test::<u128>(123, 456, 123);
    test::<u16>(456, 123, 87);
    test::<u32>(u32::MAX, 1, 0);
    test::<usize>(0xffff_ffff, 0xffff_ffff, 0);
    test::<u64>(1_000_000_000_000, 1, 0);
    test::<u64>(1_000_000_000_000, 3, 1);
    test::<u64>(1_000_000_000_000, 123, 100);
    test::<u64>(1_000_000_000_000, 0xffff_ffff, 3_567_587_560);
    test::<u128>(1_000_000_000_000_000_000_000_000, 1, 0);
    test::<u128>(1_000_000_000_000_000_000_000_000, 3, 1);
    test::<u128>(1_000_000_000_000_000_000_000_000, 123, 37);
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        3_167_723_695,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_234_567_890_987,
        530_068_894_399,
    );
    test::<u128>(
        253_640_751_230_376_270_397_812_803_167,
        2_669_936_877_441,
        1_520_301_762_334,
    );
    test::<u64>(
        3_768_477_692_975_601,
        11_447_376_614_057_827_956,
        3_768_477_692_975_601,
    );
    test::<u64>(
        3_356_605_361_737_854,
        3_081_095_617_839_357,
        275_509_743_898_497,
    );
    test::<u128>(
        1_098_730_198_198_174_614_195,
        953_382_298_040_157_850_476,
        145_347_900_158_016_763_719,
    );
    test::<u128>(
        69_738_658_860_594_537_152_875_081_748,
        69_738_658_860_594_537_152_875_081_748,
        0,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000_000_000_000_000,
        0,
    );
    test::<u128>(0, 1_000_000_000_000_000_000_000_000, 0);
    test::<u128>(123, 1_000_000_000_000_000_000_000_000, 123);
}

#[test]
fn test_div_mod_signed() {
    fn test<T: PrimitiveSigned>(n: T, d: T, r: T) {
        assert_eq!(n.mod_op(d), r);

        let mut mut_n = n;
        mut_n.mod_assign(d);
        assert_eq!(mut_n, r);
    };
    test::<i8>(0, 1, 0);
    test::<i16>(0, 123, 0);
    test::<i32>(1, 1, 0);
    test::<i64>(123, 1, 0);
    test::<i128>(123, 123, 0);
    test::<isize>(123, 456, 123);
    test::<i16>(456, 123, 87);
    test::<i64>(0xffff_ffff, 1, 0);
    test::<i64>(0xffff_ffff, 0xffff_ffff, 0);
    test::<i64>(1_000_000_000_000, 1, 0);
    test::<i64>(1_000_000_000_000, 3, 1);
    test::<i64>(1_000_000_000_000, 123, 100);
    test::<i64>(1_000_000_000_000, 0xffff_ffff, 3_567_587_560);
    test::<i128>(1_000_000_000_000_000_000_000_000, 1, 0);
    test::<i128>(1_000_000_000_000_000_000_000_000, 3, 1);
    test::<i128>(1_000_000_000_000_000_000_000_000, 123, 37);
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        3_167_723_695,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_234_567_890_987,
        530_068_894_399,
    );
    test::<i128>(
        253_640_751_230_376_270_397_812_803_167,
        2_669_936_877_441,
        1_520_301_762_334,
    );
    test::<i128>(
        3_768_477_692_975_601,
        11_447_376_614_057_827_956,
        3_768_477_692_975_601,
    );
    test::<i64>(
        3_356_605_361_737_854,
        3_081_095_617_839_357,
        275_509_743_898_497,
    );
    test::<i128>(
        1_098_730_198_198_174_614_195,
        953_382_298_040_157_850_476,
        145_347_900_158_016_763_719,
    );
    test::<i128>(
        69_738_658_860_594_537_152_875_081_748,
        69_738_658_860_594_537_152_875_081_748,
        0,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000_000_000_000_000,
        0,
    );
    test::<i128>(0, 1_000_000_000_000_000_000_000_000, 0);
    test::<i128>(123, 1_000_000_000_000_000_000_000_000, 123);

    test::<i8>(0, -1, 0);
    test::<i16>(0, -123, 0);
    test::<i32>(1, -1, 0);
    test::<i64>(123, -1, 0);
    test::<i128>(123, -123, 0);
    test::<isize>(123, -456, -333);
    test::<i16>(456, -123, -36);
    test::<i64>(0xffff_ffff, -1, 0);
    test::<i64>(0xffff_ffff, -0xffff_ffff, 0);
    test::<i64>(1_000_000_000_000, -1, 0);
    test::<i64>(1_000_000_000_000, -3, -2);
    test::<i64>(1_000_000_000_000, -123, -23);
    test::<i64>(1_000_000_000_000, -0xffff_ffff, -727_379_735);
    test::<i128>(1_000_000_000_000_000_000_000_000, -1, 0);
    test::<i128>(1_000_000_000_000_000_000_000_000, -3, -2);
    test::<i128>(1_000_000_000_000_000_000_000_000, -123, -86);
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        -1_127_243_600,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_234_567_890_987,
        -704_498_996_588,
    );
    test::<i128>(
        253_640_751_230_376_270_397_812_803_167,
        -2_669_936_877_441,
        -1_149_635_115_107,
    );
    test::<i128>(
        3_768_477_692_975_601,
        -11_447_376_614_057_827_956,
        -11_443_608_136_364_852_355,
    );
    test::<i64>(
        3_356_605_361_737_854,
        -3_081_095_617_839_357,
        -2_805_585_873_940_860,
    );
    test::<i128>(
        1_098_730_198_198_174_614_195,
        -953_382_298_040_157_850_476,
        -808_034_397_882_141_086_757,
    );
    test::<i128>(
        69_738_658_860_594_537_152_875_081_748,
        -69_738_658_860_594_537_152_875_081_748,
        0,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000_000_000_000_000,
        0,
    );
    test::<i128>(0, -1_000_000_000_000_000_000_000_000, 0);
    test::<i128>(
        123,
        -1_000_000_000_000_000_000_000_000,
        -999_999_999_999_999_999_999_877,
    );

    test::<i8>(-1, 1, 0);
    test::<i16>(-123, 1, 0);
    test::<i32>(-123, 123, 0);
    test::<i64>(-123, 456, 333);
    test::<isize>(-456, 123, 36);
    test::<i64>(-0xffff_ffff, -1, 0);
    test::<i64>(-0xffff_ffff, 0xffff_ffff, 0);
    test::<i64>(-1_000_000_000_000, 1, 0);
    test::<i64>(-1_000_000_000_000, 3, 2);
    test::<i64>(-1_000_000_000_000, 123, 23);
    test::<i64>(-1_000_000_000_000, 0xffff_ffff, 727_379_735);
    test::<i128>(-1_000_000_000_000_000_000_000_000, 1, 0);
    test::<i128>(-1_000_000_000_000_000_000_000_000, 3, 2);
    test::<i128>(-1_000_000_000_000_000_000_000_000, 123, 86);
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        1_127_243_600,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_234_567_890_987,
        704_498_996_588,
    );
    test::<i128>(
        -253_640_751_230_376_270_397_812_803_167,
        2_669_936_877_441,
        1_149_635_115_107,
    );
    test::<i128>(
        -3_768_477_692_975_601,
        11_447_376_614_057_827_956,
        11_443_608_136_364_852_355,
    );
    test::<i64>(
        -3_356_605_361_737_854,
        3_081_095_617_839_357,
        2_805_585_873_940_860,
    );
    test::<i128>(
        -1_098_730_198_198_174_614_195,
        953_382_298_040_157_850_476,
        808_034_397_882_141_086_757,
    );
    test::<i128>(
        -69_738_658_860_594_537_152_875_081_748,
        69_738_658_860_594_537_152_875_081_748,
        0,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000_000_000_000_000,
        0,
    );
    test::<i128>(
        -123,
        1_000_000_000_000_000_000_000_000,
        999_999_999_999_999_999_999_877,
    );

    test::<i8>(-1, -1, 0);
    test::<i16>(-123, -1, 0);
    test::<i32>(-123, -123, 0);
    test::<i64>(-123, -456, -123);
    test::<isize>(-456, -123, -87);
    test::<i128>(-0xffff_ffff, -1, 0);
    test::<i64>(-0xffff_ffff, -0xffff_ffff, 0);
    test::<i64>(-1_000_000_000_000, -1, 0);
    test::<i64>(-1_000_000_000_000, -3, -1);
    test::<i64>(-1_000_000_000_000, -123, -100);
    test::<i64>(-1_000_000_000_000, -0xffff_ffff, -3_567_587_560);
    test::<i128>(-1_000_000_000_000_000_000_000_000, -1, 0);
    test::<i128>(-1_000_000_000_000_000_000_000_000, -3, -1);
    test::<i128>(-1_000_000_000_000_000_000_000_000, -123, -37);
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        -3_167_723_695,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_234_567_890_987,
        -530_068_894_399,
    );
    test::<i128>(
        -253_640_751_230_376_270_397_812_803_167,
        -2_669_936_877_441,
        -1_520_301_762_334,
    );
    test::<i128>(
        -3_768_477_692_975_601,
        -11_447_376_614_057_827_956,
        -3_768_477_692_975_601,
    );
    test::<i64>(
        -3_356_605_361_737_854,
        -3_081_095_617_839_357,
        -275_509_743_898_497,
    );
    test::<i128>(
        -1_098_730_198_198_174_614_195,
        -953_382_298_040_157_850_476,
        -145_347_900_158_016_763_719,
    );
    test::<i128>(
        -69_738_658_860_594_537_152_875_081_748,
        -69_738_658_860_594_537_152_875_081_748,
        0,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000_000_000_000_000,
        0,
    );
    test::<i128>(-123, -1_000_000_000_000_000_000_000_000, -123);

    test::<i8>(-128, -1, 0);
}

fn mod_fail_helper<T: PrimitiveInteger>() {
    assert_panic!(T::ONE.mod_op(T::ZERO));
    assert_panic!(T::ONE.mod_assign(T::ZERO));
}

#[test]
pub fn mod_fail() {
    apply_fn_to_primitive_ints!(mod_fail_helper);
}

#[test]
fn test_neg_mod() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, r: T) {
        assert_eq!(n.neg_mod(d), r);

        let mut mut_n = n;
        mut_n.neg_mod_assign(d);
        assert_eq!(mut_n, r);
    };
    test::<u8>(0, 1, 0);
    test::<u16>(0, 123, 0);
    test::<u32>(1, 1, 0);
    test::<u64>(123, 1, 0);
    test::<u128>(123, 123, 0);
    test::<usize>(123, 456, 333);
    test::<u16>(456, 123, 36);
    test::<u64>(0xffff_ffff, 1, 0);
    test::<u64>(0xffff_ffff, 0xffff_ffff, 0);
    test::<u64>(1_000_000_000_000, 1, 0);
    test::<u64>(1_000_000_000_000, 3, 2);
    test::<u64>(1_000_000_000_000, 123, 23);
    test::<u64>(1_000_000_000_000, 0xffff_ffff, 727_379_735);
    test::<u128>(1_000_000_000_000_000_000_000_000, 1, 0);
    test::<u128>(1_000_000_000_000_000_000_000_000, 3, 2);
    test::<u128>(1_000_000_000_000_000_000_000_000, 123, 86);
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        1_127_243_600,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_234_567_890_987,
        704_498_996_588,
    );
    test::<u128>(
        253_640_751_230_376_270_397_812_803_167,
        2_669_936_877_441,
        1_149_635_115_107,
    );
    test::<u64>(
        3_768_477_692_975_601,
        11_447_376_614_057_827_956,
        11_443_608_136_364_852_355,
    );
    test::<u64>(
        3_356_605_361_737_854,
        3_081_095_617_839_357,
        2_805_585_873_940_860,
    );
    test::<u128>(
        1_098_730_198_198_174_614_195,
        953_382_298_040_157_850_476,
        808_034_397_882_141_086_757,
    );
    test::<u128>(
        69_738_658_860_594_537_152_875_081_748,
        69_738_658_860_594_537_152_875_081_748,
        0,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000_000_000_000_000,
        0,
    );
    test::<u128>(0, 1_000_000_000_000_000_000_000_000, 0);
    test::<u128>(
        123,
        1_000_000_000_000_000_000_000_000,
        999_999_999_999_999_999_999_877,
    );
}

fn neg_mod_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.neg_mod(T::ZERO));
    assert_panic!(T::ONE.neg_mod_assign(T::ZERO));
}

#[test]
pub fn neg_mod_fail() {
    apply_fn_to_unsigneds!(neg_mod_fail_helper);
}

#[test]
fn test_ceiling_mod() {
    fn test<T: PrimitiveSigned>(n: T, d: T, r: T) {
        assert_eq!(n.ceiling_mod(d), r);

        let mut mut_n = n;
        mut_n.ceiling_mod_assign(d);
        assert_eq!(mut_n, r);
    };
    test::<i8>(0, 1, 0);
    test::<i16>(0, 123, 0);
    test::<i32>(1, 1, 0);
    test::<i64>(123, 1, 0);
    test::<i128>(123, 123, 0);
    test::<isize>(123, 456, -333);
    test::<i16>(456, 123, -36);
    test::<i64>(0xffff_ffff, 1, 0);
    test::<i64>(0xffff_ffff, 0xffff_ffff, 0);
    test::<i64>(1_000_000_000_000, 1, 0);
    test::<i64>(1_000_000_000_000, 3, -2);
    test::<i64>(1_000_000_000_000, 123, -23);
    test::<i64>(1_000_000_000_000, 0xffff_ffff, -727_379_735);
    test::<i128>(1_000_000_000_000_000_000_000_000, 1, 0);
    test::<i128>(1_000_000_000_000_000_000_000_000, 3, -2);
    test::<i128>(1_000_000_000_000_000_000_000_000, 123, -86);
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        -1_127_243_600,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_234_567_890_987,
        -704_498_996_588,
    );
    test::<i128>(
        253_640_751_230_376_270_397_812_803_167,
        2_669_936_877_441,
        -1_149_635_115_107,
    );
    test::<i128>(
        3_768_477_692_975_601,
        11_447_376_614_057_827_956,
        -11_443_608_136_364_852_355,
    );
    test::<i64>(
        3_356_605_361_737_854,
        3_081_095_617_839_357,
        -2_805_585_873_940_860,
    );
    test::<i128>(
        1_098_730_198_198_174_614_195,
        953_382_298_040_157_850_476,
        -808_034_397_882_141_086_757,
    );
    test::<i128>(
        69_738_658_860_594_537_152_875_081_748,
        69_738_658_860_594_537_152_875_081_748,
        0,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000_000_000_000_000,
        0,
    );
    test::<i128>(0, 1_000_000_000_000_000_000_000_000, 0);
    test::<i128>(
        123,
        1_000_000_000_000_000_000_000_000,
        -999_999_999_999_999_999_999_877,
    );

    test::<i8>(0, -1, 0);
    test::<i16>(0, -123, 0);
    test::<i32>(1, -1, 0);
    test::<i64>(123, -1, 0);
    test::<i128>(123, -123, 0);
    test::<isize>(123, -456, 123);
    test::<i16>(456, -123, 87);
    test::<i64>(0xffff_ffff, -1, 0);
    test::<i64>(0xffff_ffff, -0xffff_ffff, 0);
    test::<i64>(1_000_000_000_000, -1, 0);
    test::<i64>(1_000_000_000_000, -3, 1);
    test::<i64>(1_000_000_000_000, -123, 100);
    test::<i64>(1_000_000_000_000, -0xffff_ffff, 3_567_587_560);
    test::<i128>(1_000_000_000_000_000_000_000_000, -1, 0);
    test::<i128>(1_000_000_000_000_000_000_000_000, -3, 1);
    test::<i128>(1_000_000_000_000_000_000_000_000, -123, 37);
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        3_167_723_695,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_234_567_890_987,
        530_068_894_399,
    );
    test::<i128>(
        253_640_751_230_376_270_397_812_803_167,
        -2_669_936_877_441,
        1_520_301_762_334,
    );
    test::<i128>(
        3_768_477_692_975_601,
        -11_447_376_614_057_827_956,
        3_768_477_692_975_601,
    );
    test::<i64>(
        3_356_605_361_737_854,
        -3_081_095_617_839_357,
        275_509_743_898_497,
    );
    test::<i128>(
        1_098_730_198_198_174_614_195,
        -953_382_298_040_157_850_476,
        145_347_900_158_016_763_719,
    );
    test::<i128>(
        69_738_658_860_594_537_152_875_081_748,
        -69_738_658_860_594_537_152_875_081_748,
        0,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000_000_000_000_000,
        0,
    );
    test::<i128>(0, -1_000_000_000_000_000_000_000_000, 0);
    test::<i128>(123, -1_000_000_000_000_000_000_000_000, 123);

    test::<i8>(-1, 1, 0);
    test::<i16>(-123, 1, 0);
    test::<i32>(-123, 123, 0);
    test::<i64>(-123, 456, -123);
    test::<i128>(-456, 123, -87);
    test::<isize>(-0xffff_ffff, 1, 0);
    test::<i64>(-0xffff_ffff, 0xffff_ffff, 0);
    test::<i64>(-1_000_000_000_000, 1, 0);
    test::<i64>(-1_000_000_000_000, 3, -1);
    test::<i64>(-1_000_000_000_000, 123, -100);
    test::<i64>(-1_000_000_000_000, 0xffff_ffff, -3_567_587_560);
    test::<i128>(-1_000_000_000_000_000_000_000_000, 1, 0);
    test::<i128>(-1_000_000_000_000_000_000_000_000, 3, -1);
    test::<i128>(-1_000_000_000_000_000_000_000_000, 123, -37);
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        -3_167_723_695,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_234_567_890_987,
        -530_068_894_399,
    );
    test::<i128>(
        -253_640_751_230_376_270_397_812_803_167,
        2_669_936_877_441,
        -1_520_301_762_334,
    );
    test::<i128>(
        -3_768_477_692_975_601,
        11_447_376_614_057_827_956,
        -3_768_477_692_975_601,
    );
    test::<i64>(
        -3_356_605_361_737_854,
        3_081_095_617_839_357,
        -275_509_743_898_497,
    );
    test::<i128>(
        -1_098_730_198_198_174_614_195,
        953_382_298_040_157_850_476,
        -145_347_900_158_016_763_719,
    );
    test::<i128>(
        -69_738_658_860_594_537_152_875_081_748,
        69_738_658_860_594_537_152_875_081_748,
        0,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000_000_000_000_000,
        0,
    );
    test::<i128>(0, 1_000_000_000_000_000_000_000_000, 0);
    test::<i128>(-123, 1_000_000_000_000_000_000_000_000, -123);

    test::<i8>(-1, -1, 0);
    test::<i16>(-123, -1, 0);
    test::<i32>(-123, -123, 0);
    test::<i64>(-123, -456, 333);
    test::<i128>(-456, -123, 36);
    test::<isize>(-0xffff_ffff, -1, 0);
    test::<i64>(-0xffff_ffff, -0xffff_ffff, 0);
    test::<i64>(-1_000_000_000_000, -1, 0);
    test::<i64>(-1_000_000_000_000, -3, 2);
    test::<i64>(-1_000_000_000_000, -123, 23);
    test::<i64>(-1_000_000_000_000, -0xffff_ffff, 727_379_735);
    test::<i128>(-1_000_000_000_000_000_000_000_000, -1, 0);
    test::<i128>(-1_000_000_000_000_000_000_000_000, -3, 2);
    test::<i128>(-1_000_000_000_000_000_000_000_000, -123, 86);
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        1_127_243_600,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_234_567_890_987,
        704_498_996_588,
    );
    test::<i128>(
        -253_640_751_230_376_270_397_812_803_167,
        -2_669_936_877_441,
        1_149_635_115_107,
    );
    test::<i128>(
        -3_768_477_692_975_601,
        -11_447_376_614_057_827_956,
        11_443_608_136_364_852_355,
    );
    test::<i64>(
        -3_356_605_361_737_854,
        -3_081_095_617_839_357,
        2_805_585_873_940_860,
    );
    test::<i128>(
        -1_098_730_198_198_174_614_195,
        -953_382_298_040_157_850_476,
        808_034_397_882_141_086_757,
    );
    test::<i128>(
        -69_738_658_860_594_537_152_875_081_748,
        -69_738_658_860_594_537_152_875_081_748,
        0,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000_000_000_000_000,
        0,
    );
    test::<i128>(0, -1_000_000_000_000_000_000_000_000, 0);
    test::<i128>(
        -123,
        -1_000_000_000_000_000_000_000_000,
        999_999_999_999_999_999_999_877,
    );

    test::<i8>(-128, -1, 0);
}

fn ceiling_mod_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::ONE.ceiling_mod(T::ZERO));
    assert_panic!(T::ONE.ceiling_mod_assign(T::ZERO));
}

#[test]
pub fn ceiling_mod_fail() {
    apply_fn_to_signeds!(ceiling_mod_fail_helper);
}
