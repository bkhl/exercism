pub fn collatz(mut n: u64) -> Result<u64, &'static str> {
    if n < 1 {
        return Err("n must be a positive integer")
    }

    let mut i = 0_u64;

    while n != 1 {
        i += 1;
        n = match n % 2 {
            0 => n / 2,
            _ => n * 3 + 1
        }
    }

    return Ok(i)
}