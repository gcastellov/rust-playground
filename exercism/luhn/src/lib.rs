pub fn is_valid(code: &str) -> bool {
    let trimmed_code = code.split_ascii_whitespace().collect::<String>();

    let input = trimmed_code.chars().rev().map(|c| char::to_digit(c, 10));

    if trimmed_code.len() <= 1 || input.clone().any(|c| c.is_none()) {
        return false;
    }

    input
        .filter_map(|n| n)
        .enumerate()
        .map(|(i, c)| {
            match (i + 1) % 2 {
                0 => {
                    let mut num = c * 2;
                    if num > 9 {
                        num = num - 9
                    }
                    num
                },
                _ => c
            }
        })
        .fold(0, |a, b| a + b)
        % 10
        == 0
}
