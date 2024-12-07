// If we list all the natural numbers below $10$ that are multiples 
// of $3$ or $5$, we get $3, 5, 6$ and $9$. The sum of these multiples 
// is $23$.
// Find the sum of all the multiples of $3$ or $5$ below $1000$.

fn _is_multiple_3_or_5(n: u32) -> bool {
    (n % 3 == 0) || (n % 5 == 0)
}

fn _find_sum_of_all_multiples_below_n(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n {
        if _is_multiple_3_or_5(i) {
            sum+=i;
        }
    }
    sum
}

#[allow(dead_code)]
pub fn answer() -> u32 {
    let n = 1000;
    _find_sum_of_all_multiples_below_n(n)
}