pub fn nth(mut count: u64) -> Result<u64, u64> {
    if count == 0 {
        return Err(count)
    }

    let mut test = 1u64;

    'outer: loop {
        test += 1;

        for test_divisor in 2..test {
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