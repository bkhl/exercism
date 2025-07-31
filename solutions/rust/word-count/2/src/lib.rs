use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    for word in words.split_whitespace().filter_map(normalize) {
        *counts.entry(word).or_insert(0) += 1;
    }

    counts
}

fn normalize(s: &str) -> Option<String> {
    let token: String = s.chars().filter(|c| c.is_alphanumeric()).collect();

    if token.is_empty() {
        None
    } else {
        Some(token.to_lowercase())
    }
}