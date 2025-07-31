pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut result = 0;

    'outer: for i in 0..limit {
        for factor in factors {
            if *factor < 1 {
                continue;
            }
            if i % *factor == 0 {
                result += i;
                continue 'outer;
            }
        }
    }

    result
}
