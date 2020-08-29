use std::char;

const DEFAULT_POSITIONS_TO_TAKE: usize = 3;
const POSITIONS_TO_TAKE: usize = 2;
const WILDCARD: char = '*';
const EMPTY: char = ' ';

pub fn annotate(minefield: &[&str]) -> Vec<String> {

    minefield
        .iter()
        .map(|row_str| row_str.to_string())
        .enumerate()
        .map(|(row_index, row)| {
            row.char_indices()
                .map(|(col_index, current_char)| match current_char {
                    ' ' => {
                        let mut r_take = POSITIONS_TO_TAKE;
                        let mut c_take = POSITIONS_TO_TAKE;
                        let mut r_index = row_index;
                        let mut c_index = col_index;

                        if let Some(r_start) = row_index.checked_sub(1) {
                            r_take = DEFAULT_POSITIONS_TO_TAKE;
                            r_index = r_start;
                        }
                        if let Some(c_start) = col_index.checked_sub(1) {
                            c_take = DEFAULT_POSITIONS_TO_TAKE;
                            c_index = c_start;
                        }

                        let count = &minefield[r_index..]
                            .iter()
                            .take(r_take)
                            .map(|r| {
                                r[c_index..]
                                    .chars()
                                    .take(c_take)
                                    .filter(|c| *c == WILDCARD)
                                    .count()
                            })
                            .fold(0, |r, s| r + s as u32);

                        match count {
                            0 => EMPTY,
                            _ => char::from_digit(*count, 10).unwrap(),
                        }
                    }
                    _ => WILDCARD,
                })
                .collect()
        })
        .collect()
}
