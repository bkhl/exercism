use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word
        .chars()
        .map(|c| c.to_lowercase())
        .flatten()
        .collect::<String>();

    let mut word_letter_counts: HashMap<char, i32> = HashMap::new();
    for letter in lowercase_word.chars() {
        *word_letter_counts.entry(letter).or_insert(0) += 1
    }

    possible_anagrams
        .iter()
        .map(|&possible_anagram| possible_anagram)
        .filter(|possible_anagram| {
            let lowercase_possible_anagram = possible_anagram
                .chars()
                .map(|c| c.to_lowercase())
                .flatten()
                .collect::<String>();

            if lowercase_possible_anagram == lowercase_word {
                return false;
            }

            let mut possible_anagram_letter_counts: HashMap<char, i32> = HashMap::new();
            for letter in lowercase_possible_anagram.chars() {
                *possible_anagram_letter_counts.entry(letter).or_insert(0) += 1
            }

            possible_anagram_letter_counts == word_letter_counts
        })
        .collect()
}
