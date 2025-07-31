pub fn collatz(mut n: u64) -> Result<u64, &'static str> {
    if n < 1 {
        return Err("n must be a positive integer")
    }

    let mut count = 0_u64;

    loop {
        if n == 1 {
            return Ok(count)
        }

        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }

        count += 1;
    }
}