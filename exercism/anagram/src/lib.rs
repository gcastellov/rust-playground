use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    possible_anagrams
        .iter()
        .filter_map(|item| -> Option<&str> {
            let lower_item = item.to_lowercase();
            if lower_item != word_lowercase {
                let item_iter = get_char_ocurrences(&word_lowercase, &lower_item);
                let word_iter = get_char_ocurrences(&lower_item, &word_lowercase);
                let intersection_count = item_iter.intersection(&word_iter).count();
                if intersection_count == item_iter.len() && intersection_count == word_iter.len() {                
                    return Some(item);
                }
            }

            None
        })
        .collect()
}

fn get_char_ocurrences<'a>(left: &'a str, right: &'a str) -> HashSet<(char, usize)> {
    left
        .chars()
        .map(|c|(c, right.matches(c).count()))
        .collect::<HashSet<(char, usize)>>()
}
