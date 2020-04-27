pub fn square_of_sum(n: u32) -> u32 {
    let sum: u32 = (1..=n).fold(0, |a,b| a + b);    
    u32::pow(sum, 2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).fold(0, |a,b| a + u32::pow(b, 2))
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
