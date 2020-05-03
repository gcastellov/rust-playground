pub fn is_armstrong_number(num: u32) -> bool {
    let num_as_str: String = num.to_string();
    let sum: u32 = num_as_str
        .chars()
        .map(|c|c.to_digit(10).unwrap())
        .map(|num|num.pow(num_as_str.len() as u32))
        .sum();

    sum == num
}
