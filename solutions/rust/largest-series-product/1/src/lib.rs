#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    if string_digits.is_empty() || span == 0 {
        return Ok(1);
    }

    let mut digits = Vec::new();
    for c in string_digits.chars() {
        if let Some(digit) = c.to_digit(10) {
            digits.push(digit as u64);
        } else {
            return Err(Error::InvalidDigit(c));
        }
    }

    Ok(digits
        .windows(span)
        .map(|w| w.into_iter().product())
        .max()
        .unwrap())
}