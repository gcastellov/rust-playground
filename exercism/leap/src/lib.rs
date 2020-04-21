const ONE_HUNDRED: u64 = 100;
const FOUR_HUNDRED: u64 = 400;
const FOUR: u64 = 4;

pub fn is_leap_year(year: u64) -> bool {
    
    let is_divisible_by_100 = year % ONE_HUNDRED == 0;
    let is_divisible_by_400 = year % FOUR_HUNDRED == 0;
    let is_divisible_by_4 = year % FOUR == 0;

    is_divisible_by_4 && (!is_divisible_by_100 || (is_divisible_by_100 && is_divisible_by_400))
}
