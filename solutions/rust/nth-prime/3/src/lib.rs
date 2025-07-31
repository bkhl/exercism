pub fn nth(mut count: usize) -> Result<usize, usize> {
    match count {
        0 => Err(count),
        1 => Ok(2),
        2 => Ok(3),
        _ => {
            count -= 2;
            let mut test = 3usize;

            loop {
                test += 2;
                if test.is_prime() {
                    count -= 1;
                    if count == 0 {
                        break
                    }
                }
            }

            Ok(test)
        }
    }
}

trait Integer {
    fn is_prime(&self) -> bool;
}

impl Integer for usize {
    fn is_prime(&self) -> bool {
        for test_divisor in 2..((*self as f64).sqrt() as usize + 1) {
            if *self % test_divisor == 0 {
                return false
            }
        }

        true
    }
}