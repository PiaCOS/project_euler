use std::time::Instant;

#[allow(dead_code)]
pub fn answer() -> u64 {
    count_prime_factor()
}

fn count_prime_factor() -> u64 {
    let mut n = 1;
    let mut triangle = 1;
    let mut max_factor = 0;


    let now = Instant::now();
    
    while max_factor < 500 {
        let mut number_of_factor = 0;
        let mut factor = 1;

        while factor < triangle {
            if triangle % factor == 0 {
                number_of_factor += 1;
            }
            factor += 1;
        }
        if max_factor < number_of_factor {
            max_factor = number_of_factor;
            let elapsed_time = now.elapsed().as_secs();
            println!("max_factor: {} -- triangle: {} -- n: {} -- in: {}s.", max_factor, triangle, n, elapsed_time);
        }
        n += 1;
        triangle += n;
    }
    0
}