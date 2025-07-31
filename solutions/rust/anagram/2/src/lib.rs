use std::collections::HashMap;
use std::collections::HashSet;

fn get_char_counts(s: String) -> HashMap<char, i32> {
    let mut word_letter_counts: HashMap<char, i32> = HashMap::new();

    for letter in s.chars() {
        *word_letter_counts.entry(letter).or_insert(0) += 1
    }

    word_letter_counts
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let word_letter_counts = get_char_counts(lowercase_word.clone());

    possible_anagrams
        .iter()
        .cloned()
        .filter(|possible_anagram| {
            let lowercase_possible_anagram = possible_anagram.to_lowercase();

            if lowercase_possible_anagram == lowercase_word {
                return false;
            }

            let possible_anagram_letter_counts = get_char_counts(lowercase_possible_anagram);
            possible_anagram_letter_counts == word_letter_counts
        })
        .collect()
}
