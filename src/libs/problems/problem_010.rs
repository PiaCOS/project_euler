use crate::libs::utils::prime_utils::sieve_of_eratosthene;

#[allow(dead_code)]
pub fn answer() -> u64 {
    let (p, _) = sieve_of_eratosthene(2_000_000);
    p.iter().fold(0, |x, y| x + y)
}