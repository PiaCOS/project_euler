// Find the first prime number where replacing some digits
// by the same number leads to the construction of a family of 
// 8 numbers.


// Notes:
// The last digit can't be replaced because it will create too many
// factor of 2. 

use crate::libs::utils::prime_utils::is_prime_simple;

#[allow(dead_code)]
pub fn answer() -> u32 {
    // let a = 123456;
    12334u32.ilog10()
}
#[allow(dead_code)]
fn is_prime(n: u32) -> bool {
    is_prime_simple(n as u64)
}
// #[allow(dead_code)]
// fn enumerate_members(n: u32, v: Vec<u32>) {
//     let n_vec = num_to_vec(n);
//     for i in v {
        
//     }
// }
// #[allow(dead_code)]
// fn size_of_family(n: u32, v: Vec<u32>) {
//     // count number of prime in a vec_to_num
// }

// fn num_to_vec(n: u32) -> Vec<u32> {
//     n.to_string()
//         .chars()
//         .map(|x| x.to_digit(10).unwrap())
//         .collect()
// }
#[allow(dead_code)]
fn vec_to_num(v: Vec<u32>) -> u32 {
    let mut n = 0;
    for (i, val) in v.iter().enumerate() {
        n += val * 10u32.pow((v.len() - i - 1) as u32)
    }
    n
}