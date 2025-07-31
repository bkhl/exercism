/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    match code.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).ok_or("Invalid character"))
        .collect()
    {
        Ok(digits) => is_valid_digit_vector(digits),
        Err(_) => false,
    }
}

/// Check a Luhn checksum (helper function),
/// taking a vector of valid digits.
fn is_valid_digit_vector(digits: Vec<u32>) -> bool {
    if digits.len() < 2 {
        return false;
    }

    let sum = digits
        .into_iter()
        .rev()
        .zip([false, true].into_iter().cycle())
        .fold(0, |acc, (i, even)| {
            if *even {
                if i < 5 {
                    acc + i * 2
                } else {
                    acc + i * 2 - 9
                }
            } else {
                acc + i
            }
        });

    sum % 10 == 0
}