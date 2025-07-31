pub fn is_valid(input: &str) -> bool {
    let digits = match input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .rev()
        .collect::<Option<Vec<u32>>>()
    {
        Some(digits) => digits,
        _ => return false,
    };

    if digits.len() < 2 {
        return false;
    }

    let sum = digits
        .into_iter()
        .zip([false, true].iter().cycle())
        .fold(0, |acc, (digit, even)| {
            if *even {
                if digit < 5 {
                    acc + digit * 2
                } else {
                    acc + digit * 2 - 9
                }
            } else {
                acc + digit
            }
        });

    sum % 10 == 0
}
