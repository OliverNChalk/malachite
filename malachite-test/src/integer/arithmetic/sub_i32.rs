use num;

pub fn num_sub_i32(mut x: num::BigInt, i: i32) -> num::BigInt {
    x = x - num::BigInt::from(i);
    x
}
