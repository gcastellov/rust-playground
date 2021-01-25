const WHITESPACE: char = ' ';
const UNDERSCORE: char = '_';
const HYPHEN: char = '-';

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(WHITESPACE)
        .flat_map(|chunk| {
            chunk.split(HYPHEN).flat_map(move |sub_chunk| {
                sub_chunk.chars().enumerate().filter_map(move |(i, c)| {
                    if i == 0 && c.is_alphabetic() {
                        Some(c)
                    } else if i > 0
                        && chunk.chars().nth(i - 1).unwrap().is_lowercase()
                        && c.is_uppercase()
                    {
                        Some(c)
                    } else if i > 0 && chunk.chars().nth(i - 1).unwrap() == UNDERSCORE {
                        Some(c)
                    } else {
                        None
                    }
                })
            })
        })
        .collect::<String>()
        .to_uppercase()
}
