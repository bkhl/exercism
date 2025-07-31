/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let toggle = [false, true].into_iter().cycle();
    let chars = code.chars().filter(|c| *c != ' ').rev();

    let mut sum = 0;
    let mut more_than_two_digits = false;

    for (c, even) in chars.zip(toggle) {
        if let Some(i) = c.to_digit(10) {
            if *even {
                more_than_two_digits = true;
                if i < 5 {
                    sum += i * 2;
                } else {
                    sum += i * 2 - 9;
                }
            } else {
                sum += i;
            }
        } else {
            return false;
        }
    }

    if more_than_two_digits {
        sum % 10 == 0
    } else {
        false
    }
}