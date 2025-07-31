use std::char::from_digit;

#[derive(PartialEq)]
enum Square {
    Mine,
    Empty(u32),
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    use Square::*;

    let mut minefield_map: Vec<Vec<Square>> = vec![];

    for (row_number, input_row) in minefield.iter().enumerate() {
        let mut output_row: Vec<Square> = vec![];

        for (column_number, c) in input_row.chars().enumerate() {
            let previous_row_start_index = if column_number > 0 {
                column_number - 1
            } else {
                0
            };

            let previous_row_end_index = if column_number < (minefield[0].len() - 1) {
                column_number + 2
            } else {
                column_number + 1
            };

            if c == '*' {
                output_row.push(Mine);

                // Update counts on previous row.
                if row_number > 0 {
                    for previous_row_square in previous_row_start_index..previous_row_end_index {
                        minefield_map[row_number - 1][previous_row_square] =
                            match minefield_map[row_number - 1][previous_row_square] {
                                Mine => Mine,
                                Empty(n) => Empty(n + 1),
                            }
                    }
                }

                // Update count based on previous square on this row.
                if column_number > 0 {
                    let previous_square = column_number - 1;
                    output_row[previous_square] = match output_row[previous_square] {
                        Mine => Mine,
                        Empty(n) => Empty(n + 1),
                    }
                }
            } else {
                // Get count of adjacent mines from previous row.
                let previous_row_count = if row_number > 0 {
                    (previous_row_start_index..previous_row_end_index)
                        .filter(|x| (minefield_map[row_number - 1][*x] == Mine))
                        .count() as u32
                } else {
                    0
                };

                // Check if previous square on line had a mine.
                let current_row_count =
                    (column_number > 0 && output_row[column_number - 1] == Mine) as u32;

                output_row.push(Empty(previous_row_count + current_row_count));
            }
        }

        minefield_map.push(output_row);
    }

    minefield_map
        .iter()
        .map(|v| {
            v.iter()
                .map(|i| match i {
                    Mine => '*',
                    Empty(0) => ' ',
                    Empty(i) => from_digit(*i, 10).unwrap(),
                })
                .collect()
        })
        .collect()
}
