struct Prime {
    current: u64,
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.current += 1;
            if self.current.is_prime() {
                return Some(self.current)
            }
        }
    }
}

fn prime() -> Prime {
    Prime { current: 1 }
}

trait Integer {
    fn is_prime(&self) -> bool;
}

impl Integer for u64 {
    fn is_prime(&self) -> bool {
        for test_divisor in 2..((*self as f64).sqrt() as u64 + 1) {
            if *self % test_divisor == 0 {
                return false
            }
        }
        true
    }
}

pub fn factors(mut n : u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut primes = prime();
    let mut p = primes.next().unwrap();

    loop {
        if n == 1 {
            break
        }

        let div = n / p;
        let rem = n % p;

        if rem == 0 {
            result.push(p);
            n = div
        } else {
           p = primes.next().unwrap()
        }
    }

    result
}