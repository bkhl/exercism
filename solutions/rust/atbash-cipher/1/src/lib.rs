/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(|c| {
            if c.is_ascii_digit() {
                Some(c)
            } else if c.is_ascii_alphabetic() {
                Some((('z' as u8) - (c.to_ascii_lowercase() as u8) + ('a' as u8)) as char)
            } else {
                None
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|l| l.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(|c| {
            if c.is_ascii_digit() {
                Some(c)
            } else if c != ' ' {
                Some((('z' as u8) - ((c as u8) - ('a' as u8))) as char)
            } else {
                None
            }
        })
        .collect()
}