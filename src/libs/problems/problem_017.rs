
pub fn answer() -> u32 {
    (1..=1000)
        .collect::<Vec<u32>>()
        .iter()
        .fold(0, |acc, &x| acc + count_letters(x))
        
    // count_letters(190)
}

fn int_to_vec(u: u32) -> Vec<u32> {
    u.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn count_letters(u: u32) -> u32 {
    let su = int_to_vec(u);
    match su.len() {
        1 => count_1(su[0]),
        2 => count_10_to_99(su),
        3 => count_100_to_999(su),
        _ => 11 // One thousand
    }
}

fn count_100_to_999(su: Vec<u32>) -> u32 {
    let h = count_1(su[0]) + 7; // two hundred

    match su[1] {
        // ?0?
        0 => match su[2] {
            0 => h, // ?0?
            _ => h + 3 + count_1(su[2]), // one hundred AND one
        },
        _ => h + 3 + count_10_to_99((su[1..]).to_vec())
    } 
}

fn count_10_to_99(su: Vec<u32>) -> u32 {
    match su[0] {
        1 => count_10_to_20(su[1]),
        4 => count_1(su[1]) + 5, // forty
        5 => count_1(su[1]) + 5, // fifty
        6 => count_1(su[1]) + 5, // sixty
        7 => count_1(su[1]) + 7, // seventy
        _ => count_1(su[1]) + 6, // ninety, 80, 20, 30
    }
}

fn count_10_to_20(u: u32) -> u32 {
    match u {
        0 => 3,
        1 => 6,
        2 => 6,
        5 => 7, // fifteen
        6 => 7, // sixteen
        7 => 9, // seventeen
        _ => 8, // nineteen, 18, 14, 13
    }
}

fn count_1(u: u32) -> u32{
    match u {
        0 => 0,
        1 => 3,
        2 => 3,
        3 => 5,
        6 => 3,
        7 => 5,
        8 => 5,
        _ => 4,
    }
}