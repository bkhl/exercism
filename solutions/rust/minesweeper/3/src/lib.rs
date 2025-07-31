use std::char::from_digit;

// Internal representation of a square, either with a mine, or a count of
// adjacent mines.
#[derive(PartialEq)]
enum Square {
    Mine,
    Empty(u32),
}

/// Given an array of string slices representing a Minesweeper map, return a
/// vector of strings, with the mines marked out, and empty squares marked with
/// a digit indicating the number of adjacent mines.
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    use Square::*;

    let mut minefield_map: Vec<Vec<Square>> = vec![];

    for (row_number, input_row) in minefield.iter().enumerate() {
        let mut output_row: Vec<Square> = vec![];

        for c in input_row.chars() {
            if c == '*' {
                handle_found_mine(&mut output_row, row_number, &mut minefield_map);
            } else {
                handle_found_empty(&mut output_row, row_number, &mut minefield_map);
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

// When a mine is found, push a Mine to the internal minefield map, and update
// adjacent mine counts on already processed squares.
fn handle_found_mine(
    output_row: &mut Vec<Square>,
    row_number: usize,
    minefield_map: &mut Vec<Vec<Square>>,
) {
    use Square::*;

    let column_number = output_row.len();

    output_row.push(Mine);

    // Update counts on previous row.
    if row_number > 0 {
        for previous_row_square in get_row_range(column_number, &minefield_map[row_number - 1]) {
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
}

// When an empty square is found, mark it with the number of adjacent mines from
// already processed squares.
fn handle_found_empty(
    output_row: &mut Vec<Square>,
    row_number: usize,
    minefield_map: &mut Vec<Vec<Square>>,
) {
    use Square::*;

    let column_number = output_row.len();

    // Get count of adjacent mines from previous row.
    let previous_row_count = if row_number > 0 {
        get_row_range(column_number, &minefield_map[row_number - 1])
            .filter(|x| (minefield_map[row_number - 1][*x] == Mine))
            .count() as u32
    } else {
        0
    };

    // Check if previous square on line had a mine.
    let current_row_count = (column_number > 0 && output_row[column_number - 1] == Mine) as u32;

    output_row.push(Empty(previous_row_count + current_row_count));
}

// Given a column number and a row, get a range to use as the horizontal window
// in which to check for adjacent mines.
fn get_row_range(column_number: usize, row: &[Square]) -> std::ops::Range<usize> {
    let start = if column_number > 0 {
        column_number - 1
    } else {
        0
    };

    let end = if column_number < (row.len() - 1) {
        column_number + 2
    } else {
        column_number + 1
    };

    start..end
}
