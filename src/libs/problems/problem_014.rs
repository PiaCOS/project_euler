
pub fn answer() -> u64 {
    collatz_brut(1_000_000)
}

pub fn collatz_next(n: u64) -> u64 {
    match n%2 == 0 {
        true => n / 2,
        false => 3*n + 1
    }
}

pub fn collatz(mut n: u64) -> u64 {
    let mut c = 1;
    while n > 1 {
        n = collatz_next(n);
        c += 1;
    }
    c
}

pub fn collatz_brut(max_n: u64) -> u64 {
    // I choose to put the terms for 0 and 1 because i want the index to match
    let mut max = 0;
    let mut num_max = 0;

    for i in 2..max_n {
        let c = collatz(i);
        if c > max {
            max = c;
            num_max = i;
        }
    }
    println!("max: {max} with num: {num_max}");
    num_max
}