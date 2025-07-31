pub fn rotate(input: &str, key: u8) -> String {
    input.chars().map(|c| transpose(key, c)).collect()
}

fn transpose(key: u8, c: char) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }

    let max_c = if c.is_ascii_lowercase() {
        'z' as u8
    } else {
        'Z' as u8
    };
    let new_c = c as u8 + key;

    if new_c > max_c {
        (new_c - 26) as char
    } else {
        new_c as char
    }
}