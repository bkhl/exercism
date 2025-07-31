extern crate itertools;
use itertools::Itertools;

pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = input
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect();

    let column_count = (chars.len() as f64).sqrt().ceil() as usize;
    let padding = (column_count.pow(2) - chars.len()) % column_count;

    let mut matrix: Vec<String> = (0..column_count)
        .map(|_| String::with_capacity(column_count))
        .collect();
    let mut row_index = (0..column_count).cycle();

    for c in chars {
        matrix.get_mut(row_index.next().unwrap()).unwrap().push(c);
    }

    for _ in 0..padding {
        matrix.get_mut(row_index.next().unwrap()).unwrap().push(' ');
    }

    matrix.iter().join(" ")
}