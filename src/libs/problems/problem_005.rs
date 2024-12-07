// <p>$2520$ is the smallest number that can be divided by each of the 
// numbers from $1$ to $10$ without any remainder.</p>
// <p>What is the smallest positive number that is evenly divisible
//  by all of the numbers from $1$ to $20$?</p>

#[allow(dead_code)]
pub fn answer() -> u32 {
    find_answer()
}

fn is_div_until(p: u32) -> bool {
    for i in 2..21 {
        if p % i != 0 {
            return false
        }
    }
    true
} 

fn find_answer() -> u32 {
    let mut n = 2520;
    while !is_div_until(n) {
        n += 2 //2+3+5+7;
    }
    n
}