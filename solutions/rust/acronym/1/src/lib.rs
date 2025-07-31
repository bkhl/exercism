use std::iter;

pub fn abbreviate(input: &str) -> String {
    input
        .split(|c| c == ' ' || c == '-')
        .map(abbreviate_word)
        .collect::<Vec<String>>()
        .join("")
}

fn abbreviate_word(word: &str) -> String {
    let (uppercase, lowercase): (Vec<char>, Vec<char>) = word.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .partition(|c| c.is_ascii_uppercase());

    if uppercase.is_empty() {
        iter::once(lowercase.first().unwrap().to_ascii_uppercase()).collect()
    } else if lowercase.is_empty() {
        iter::once(uppercase.first().unwrap()).collect()
    } else {
        uppercase.iter().collect()
    }
}