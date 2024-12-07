#[allow(dead_code)]
pub fn answer() -> u64 {
    square_of_sum(100) - sum_of_square(100)
}

fn sum_of_square(n: u64) -> u64 {
    n * (n + 1) * (2 * n + 1) / 6
}

fn square_of_sum(n: u64) -> u64 {
    (n * (n + 1) / 2).pow(2)
}