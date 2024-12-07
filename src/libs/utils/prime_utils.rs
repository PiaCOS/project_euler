
use tqdm::tqdm;

pub fn sieve_of_eratosthene(n: u64) -> (Vec<u64>, Vec<bool>) {
    let mut bool_primes = vec![true; n as usize + 1];
    bool_primes[0] = false;
    bool_primes[1] = false;
    // i know it's awful.. ;_;
    let m = ((n as f64).sqrt().ceil()) as u64;
    for i in tqdm(2..m) {

        if bool_primes[i as usize] {
            let mut j = i*i;
            while j <= n {
                bool_primes[j as usize] = false;
                j += i;
            }
        }
    }
    let mut primes = Vec::new();
    for (i, &val) in bool_primes.iter().enumerate() {
        if val {
            primes.push((i) as u64);
        }
    }
    (primes, bool_primes)
}

pub fn is_prime_simple(n: u64) -> bool {
    // all prime are of form 6 +/- 1 except 2 and 3 
    if n <= 1 {return false};
    if n <= 3 {return true};

    if n % 2 == 0 || n % 3 == 0 {return false};

    let mut i = 5;
    while i*i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false
        }
        i += 6;
    }
    true
}

pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();
    while n % 2 == 0 {
        v.push(2);
        n = n / 2;
    }
    let sq = (n as f64).sqrt().floor() as u64;
    for i in (3..(sq + 1)).step_by(2) {
        while n % i == 0 {
            v.push(i);
            n = n / i;
        }
    }
    if n > 2 {
        v.push(n);
    }
    v
}