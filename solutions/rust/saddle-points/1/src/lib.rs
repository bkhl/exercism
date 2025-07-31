pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let point = Some(&input[y][x]);

            if input[y].iter().max() == point
                && input.iter().map(|v| &v[x]).min() == point
            {
                result.push((y, x));
            }
        }
    }

    result
}