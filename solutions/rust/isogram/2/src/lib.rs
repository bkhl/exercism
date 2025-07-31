pub fn check(word: &str) -> bool {
    let mut seen = Vec::new();

    for c in word.chars() {
        if c.is_alphabetic() {
            let c_lowercase: String = c.to_lowercase().collect();
            if seen.contains(&c_lowercase) {
                return false;
            }
            seen.push(c_lowercase)
        }
    }

    true
}