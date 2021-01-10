use std::collections::HashMap;

const WHITESPACE: char = ' ';
const APOSTROPHE: char = '\'';

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut output: HashMap<String, u32> = HashMap::default();
    let chars: Vec<char> = words.chars().collect();

    chars
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if c.is_alphabetic() || c.is_numeric() {
                *c
            } else if *c == APOSTROPHE
                && chars[i - 1].is_alphabetic()
                && chars[i + 1].is_alphabetic()
            {
                *c
            } else {
                WHITESPACE
            }
        })
        .collect::<String>()
        .split(WHITESPACE)
        .filter_map(|str| {
            if str.is_empty() {
                None
            } else {
                Some(str.to_ascii_lowercase())
            }
        })
        .for_each(|word| *output.entry(word).or_insert(0) += 1);

    output
}
