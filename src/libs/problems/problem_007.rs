use crate::libs::utils::prime_utils::sieve_of_eratosthene;

#[allow(dead_code)]
pub fn answer() -> u64 {
    let (p, _) = sieve_of_eratosthene(100_000_000);
    p[10_000]
}