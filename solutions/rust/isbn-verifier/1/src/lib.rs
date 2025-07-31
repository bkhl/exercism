pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits_result = get_digits(isbn);
    match digits_result {
        Ok(digits) => is_valid_isbn_digits(digits),
        Err(_) => false,
    }
}

fn get_digits(isbn: &str) -> Result<Vec<u32>, & 'static str> {
    let mut digits = Vec::new();
    let mut x_seen = false;

    for c in isbn.chars() {
        if x_seen {
            return Err("'X' in invalid position");
        }

        match c {
            'X' => {
                x_seen = true;
                digits.push(10)
            }
            '-' => {}
            c if c.is_digit(10) => digits.push(c.to_digit(10).unwrap()),
            _ => return Err("Invalid character"),
        }
    }

    Ok(digits)
}

fn is_valid_isbn_digits(isbn: Vec<u32>) -> bool {
    if isbn.len() != 10 {
        return false;
    }

    let mut checksum = 0;
    let mut i = 10;
    for d in isbn {
        checksum += d * i;
        i -= 1;
    }

    if checksum % 11 == 0 {
        true
    } else {
        false
    }
}