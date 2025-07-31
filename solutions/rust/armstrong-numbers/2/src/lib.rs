pub fn is_armstrong_number(num: u64) -> bool {
    let mut rem = num;
    let mut digit;
    let mut digits = Vec::new();

    while rem > 0 {
        digit = rem % 10;
        rem = rem / 10;
        digits.push(digit);
    }

    let sum_of_powers = digits
        .iter()
        .fold(0, |acc, x| acc + x.pow(digits.len() as u32));

    return sum_of_powers == num;
}
