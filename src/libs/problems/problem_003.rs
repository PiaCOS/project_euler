// The prime factors of $13195$ are $5, 7, 13$ and $29$.
// What is the largest prime factor of the number $600851475143$?
use tqdm::tqdm;

use crate::libs::utils::prime_utils;
use prime_utils::sieve_of_eratosthene;


#[allow(dead_code)]
pub fn answer() -> u64 {
    let n = 600_851_475_143;
    largest_bounded_prime_factor(n, 10_000_000_000)
}

// The idea is to divide the number by the biggest number possible
pub fn largest_bounded_prime_factor(n: u64, thr: u64) -> u64 {
    let (primes, _) = sieve_of_eratosthene(thr);
    let mut biggest = 1;
    for p in tqdm(primes) {
        if n % p == 0 {
            biggest = p;
            println!("{p} is a prime factor of n")
        }
    }
    biggest
}

