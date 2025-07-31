pub fn check(word: &str) -> bool {
    let mut seen = Vec::new();

    for c in word.chars() {
        let c_lowercase: String = c.to_lowercase().collect();

        if seen.contains(&c_lowercase) {
            return false;
        } else if c.is_alphabetic() {
            seen.push(c_lowercase)
        }
    }

    true
}