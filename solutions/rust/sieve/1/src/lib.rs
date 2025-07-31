use std::collections::BTreeSet;

pub fn primes_up_to(number: u32) -> Vec<u32> {
    let upper_bound = number + 1;

    let mut numbers: BTreeSet<u32> = (2..upper_bound).collect();

    for n in 2..(upper_bound / 2) {
        if numbers.contains(&n) {
            let mut multiple = n * 2;
            while multiple < upper_bound {
                numbers.remove(&multiple);
                multiple += n;
            }
        }
    }

    numbers.into_iter().collect()
}