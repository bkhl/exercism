pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            rows: Self::calculate_rows(row_count),
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }

    fn calculate_rows(row_count: u32) -> Vec<Vec<u32>> {
        if row_count == 0 {
            return vec![];
        }

        let mut rows = vec![vec![1]];

        while (rows.len() as u32) < row_count {
            let mut new_row = vec![1];
            let previous_row = rows.last().unwrap().clone();

            for i in 1..(previous_row.len()) {
                new_row.push(previous_row[i - 1] + previous_row[i]);
            }

            new_row.push(1);
            rows.push(new_row);
        }

        rows
    }
}