pub trait LuhnValidate {
    fn valid(&self) -> bool;
}

impl LuhnValidate for str {
    fn valid(&self) -> bool {
        let without_whitespace: String = self.chars().filter(|c| !c.is_whitespace()).collect();

        if without_whitespace.len() < 2 {
            return false;
        }

        match without_whitespace.parse::<u64>() {
            Ok(number) => number == 0 || number.valid(),
            Err(_) => false,
        }
    }
}

impl LuhnValidate for u64 {
    /// Performs Luhn validation on a u64.
    ///
    /// WARNING: Can not check valid numbers consisting of only zeroes, such as `0000`.
    fn valid(&self) -> bool {
        if *self < 10 {
            return false;
        }

        let mut number = *self;
        let mut sum = 0;
        let mut toggle = [false, true].into_iter().cycle();

        while number > 0 {
            let even = toggle.next().unwrap();
            let digit = number % 10;

            number = number / 10;

            if *even {
                if digit < 5 {
                    sum += digit * 2;
                } else {
                    sum += digit * 2 - 9;
                }
            } else {
                sum += digit;
            }
        }

        sum % 10 == 0
    }
}