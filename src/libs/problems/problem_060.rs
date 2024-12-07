// The primes $3$, $7$, $109$, and $673$, are quite remarkable. 
// By taking any two primes and concatenating them in any order the 
// result will always be prime. For example, taking $7$ and $109$, 
// both $7109$ and $1097$ are prime. 

// The sum of these four primes, $792$, represents the lowest sum 
// for a set of four primes with this property.

// Find the lowest sum for a set of five primes for which any two 
// primes concatenate to produce another prime.

use crate::libs::utils::prime_utils;
use prime_utils::{sieve_of_eratosthene, is_prime_simple};

#[allow(dead_code)]
pub fn answer() -> u64 {
    let gen_num = 3_000_000_000;
    let (primes, bool_primes) = sieve_of_eratosthene(gen_num);
    let test = vec![3, 7, 109, 673];
    println!("{}", is_answer(&test, &bool_primes, gen_num));

    let mut min_sum = 50_000;

    let mut first = 5;
    while primes[first] < min_sum {
        let mut i = 4;
        while i < first {
            let v = vec![primes[first], primes[i]];

            if is_answer(&v, &bool_primes, gen_num) {

                let mut j = 3;
                while j < i {
                    let v = vec![primes[first], primes[i], primes[j]];

                    if is_answer(&v, &bool_primes, gen_num) {

                        let mut k = 2;
                        while k < j {
                            let v = vec![primes[first], primes[i], primes[j], primes[k]];

                            if is_answer(&v, &bool_primes, gen_num) {

                                let mut last = 1;
                                while last < k {
                                    let v = vec![primes[first], primes[i], primes[j], primes[k], primes[last]];
                                    let current_sum = v.iter().sum::<u64>();
                                
                                    if is_answer(&v, &bool_primes, gen_num) {
                                        if current_sum < min_sum {
                                            min_sum = current_sum;
                                            println!("---> new best sum: {} -- obtained with {:?}", current_sum, v)
                                        } else {
                                            println!("------------> sum: {} -- obtained with {:?}", current_sum, v);
                                        }
                                    }
                                    last+=1;
                                }
                            } 
                            k+=1;
                        }
                    }
                    j+=1;
                }
            }
            i+=1;
        }
        first+=1;
    }

    min_sum

}


fn concat_prime(left: u64, right: u64) -> u64 {
    left * 10u64.pow(right.ilog10()+1) + right
}

fn is_answer(list_prime: &Vec<u64>, bool_primes: &Vec<bool>, gen_num: u64) -> bool {
    for i in list_prime {
        for j in list_prime {
            if i != j {
                let c = concat_prime(*i, *j);
                if c >= gen_num {
                    if !is_prime_simple(c) {
                        return false
                    }
                } else {
                    if !bool_primes[c as usize] {
                        return false
                    }
                }
            }
        }
    }
    true
}