extern crate num_bigint;
use num_bigint::BigUint;

pub fn answer() -> u32 {
    let x = BigUint::from(2u64);
    let s = format!("{}", x.pow(1000));

    s.chars().map(|x| x.to_digit(10).unwrap()).fold(0, |acc, x| acc + x)
}

// ideas:
// - do the vec thing 
// - find crate for those kind of problems !!!! bigint=bigwin