#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if first_list.len() < second_list.len() && is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.is_empty() {
        return true;
    }

    let mut slice = second_list;
    while let Some(s) = get_slice(first_list, slice) {
        if s.len() >= first_list.len() && &s[0..first_list.len()] == first_list {
            return true;
        }
        slice = &s[1..];
    }

    false
}

fn get_slice<'a, T: PartialEq>(first_list: &'a [T], slice: &'a [T]) -> Option<&'a [T]> {
    slice.iter().position(|p|p == &first_list[0]).map(|start| &slice[start..])
}
