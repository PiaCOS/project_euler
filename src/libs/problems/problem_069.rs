
pub fn answer() -> usize {
    find_n(1_000_000 as usize)
}

// We want to find n such thar n/phi(n) is maximized.
// This means minimizing phi and maximizing n
// We can just multiply all primes while being less than the limit 

fn find_n(l: usize) -> usize {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 21, 23, 29, 31];
    let mut n = 1;
    let mut k = 0;

    while primes[k] * n <= l {
        n *= primes[k];
        k += 1;
    }
    n
}


