pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let row_size = row_count as usize;
        let mut rows = Vec::with_capacity(row_size);

        (0..row_size)
            .for_each(|row_index|{
                let mut current_row = vec![0u32; row_index + 1];
                for col_index in 0..=row_index {
                    current_row[col_index] = match col_index {
                        0 => 1,
                        _ => {
                            let last_row: &Vec<u32> = &rows[row_index - 1];
                            let left = last_row[col_index-1];
                            let right = if col_index < last_row.len() { last_row[col_index] } else { 0 };
                            left + right
                        }
                    }
                }
                rows.push(current_row);
            });
        
        PascalsTriangle { rows:  rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut output: Vec<Vec<u32>> = Vec::new();
        output.extend_from_slice(self.rows.as_slice());
        output
    }
}
