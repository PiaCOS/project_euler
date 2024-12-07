#[allow(dead_code)]
pub fn answer() -> u32 {
    find_triplet(1000)
}

fn is_pythagorean_triplet(a: u32, b: u32) -> bool {
    ((a*a + b*b) as f64).sqrt().fract() == 0.0
}

fn find_triplet(ceil: u32) -> u32 {
    for i in 2..ceil {
        for j in 1..i {
            if is_pythagorean_triplet(i, j) {
                let k = ((i*i + j*j )as f64).sqrt() as u32;
                if i + j + k == 1000 {
                    println!("{i}, {j}, {k}");
                    return i * j * k
                }
            }
        }
    }
    return 0
}