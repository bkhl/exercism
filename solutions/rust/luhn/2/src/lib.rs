/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let characters: &Vec<Option<u32>> = &code.chars()
        .rev()
        .filter(|c| *c != ' ')
        .map(|c| c.to_digit(10))
        .collect();

    if characters.len() < 2 {
        return false;
    }

    if characters.into_iter().any(|x| *x == None) {
        return false;
    }

    let toggle = [false, true].into_iter().cycle();

    let sum = characters
        .into_iter()
        .map(|x| x.unwrap())
        .zip(toggle)
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