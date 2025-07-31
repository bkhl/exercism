use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let mut used_letters = HashSet::new();
    for letter in sentence.chars() {
        if letter.is_ascii_alphabetic() {
            used_letters.insert(letter.to_ascii_lowercase());
        }
    }
    used_letters.len() == 26
}