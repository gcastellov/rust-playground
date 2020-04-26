pub fn build_proverb(list: &[&str]) -> String {

    let choose_verse = |index: usize, item: &str| -> String {
        if index < list.len() - 1 {
            format!("For want of a {} the {} was lost.\n", item, list[index+1])
        }
        else {
            format!("And all for the want of a {}.", list[0])
        }
    };

    list
        .iter()
        .enumerate()
        .map(|(index, item)|choose_verse(index, item))
        .collect()
}
