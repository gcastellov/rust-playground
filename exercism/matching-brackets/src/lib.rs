pub fn brackets_are_balanced(string: &str) -> bool {
    let mut char_stack: Vec<char> = Vec::new();
    let allowed_chars = vec!['{', '[', '(', '}', ']', ')'];
    let left_chars = &allowed_chars[0..3];
    let right_chars = &allowed_chars[3..];

    string
        .chars()
        .all(|current|{
            if left_chars.contains(&current) {
                char_stack.push(current);
                return true;
            }
            else if right_chars.contains(&current) {
                if let Some(c) = char_stack.pop() {
                    if get_expected_match(c) == current {
                        return true;
                    }
                }
            }
            else {
                return true;
            }
            false
        }) && char_stack.is_empty()
}

fn get_expected_match(c: char) -> char {
    match c {
        '{' => '}',
        '[' => ']',
        '(' => ')',
        _ => panic!("not supported char")
    }
}