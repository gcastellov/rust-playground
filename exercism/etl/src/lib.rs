use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h
        .iter()
        .map(|(score, chars)| chars
            .iter()
            .map(move |c| {
                let mut c_lower = *c;
                c_lower.make_ascii_lowercase();
                (c_lower, *score)
            }))
        .flatten()
        .collect()
}
