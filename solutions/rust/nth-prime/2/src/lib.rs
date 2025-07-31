pub fn nth(mut count: u32) -> Result<u32, u32> {
    match count {
        0 => Err(count),
        1 => Ok(2),
        2 => Ok(3),
        _ => {
            count -= 2;
            let mut test = 3u32;

            'outer: loop {
                test += 2;

                for test_divisor in 2..((test as f64).sqrt() as u32 + 1) {
                    if test % test_divisor == 0 {
                        continue 'outer
                    }
                }

                count -= 1;
                if count == 0 {
                    break 'outer
                }
            }

            Ok(test)
        }
    }
}