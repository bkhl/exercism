extern crate rand;
use rand::{thread_rng, Rng};

pub fn encode(key: &str, s: &str) -> Option<String> {
    if !valid_key(key) {
        return None;
    }

    let result = s.chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| (c as u8 + k as u8 - 'a' as u8) as char)
        .map(|c| if c > 'z' { (c as u8 - 26) as char } else { c })
        .collect();

    Some(result)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !valid_key(key) {
        return None;
    }

    let result = s.chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| (c as u8 + 'a' as u8 - k as u8) as char)
        .map(|c| if c < 'a' { (c as u8 + 26) as char } else { c })
        .collect();

    Some(result)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();

    let key_length = rng.gen_range(100, 1000);
    let key: String = (0..key_length)
        .map(|_| rng.gen_range('a' as u8, 'z' as u8) as char)
        .collect();

    (key.clone(), encode(&key, s).unwrap())
}

fn valid_key(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }

    if key.chars().any(|c| !c.is_ascii_lowercase()) {
        return false;
    }

    true
}