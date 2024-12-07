// A palindromic number reads the same both ways. 
// The largest palindrome made from the product of 
// two 2-digit numbers is 9009 = 91 * 99$.
// Find the largest palindrome made from the product of two 3-digit numbers.

struct Palindrome {
    factors: (u32, u32),
    prod: u32,
}


#[allow(dead_code)]
pub fn answer() -> u32 {
    biggest_palindrome()
}

fn num_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect()
}

fn is_palindrome(n: u32) -> bool {
    let v = num_to_vec(n);
    let mut p1 = 0;
    let mut p2 = v.len() - 1;
    while p1 < p2 {
        if v[p1] != v[p2] {return false};
        p1 += 1;
        p2 -= 1;
    }
    true
}

fn biggest_palindrome() -> u32 {
    let mut max_palindrome = Palindrome{factors: (0,0), prod: 0};
    for i in 100..1000 {
        for j in (i+1)..1000 {
            let n = i * j;
            if is_palindrome(n) && n > max_palindrome.prod {
                max_palindrome = Palindrome{factors: (i,j), prod: n};
            }
        }
    }
    println!("{:?}", max_palindrome.factors);
    max_palindrome.prod
}