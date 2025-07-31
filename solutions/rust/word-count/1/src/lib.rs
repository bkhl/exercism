use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut word = String::new();
    let mut chars = words.chars().peekable();
    let mut counts: HashMap<String, u32> = HashMap::new();

    loop {
        match chars.next() {
            Some(c) => {
                if c.is_alphanumeric() {
                    word.push(c.to_ascii_lowercase());
                }

                if (!c.is_alphanumeric() || chars.peek() == None) && (!word.is_empty()) {
                    let counter = counts.entry(word.clone()).or_insert(0);
                    *counter += 1;
                    word.clear()
                }
            }
            None => {
                break;
            }
        }
    }

    counts
}