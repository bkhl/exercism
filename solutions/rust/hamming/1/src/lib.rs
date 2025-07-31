pub fn hamming_distance(a: &str, b: &str) -> Result<u32, &'static str> {
    if a.len() != b.len() {
        return Err("Strands are of different length.");
    }

    let distance = a.chars()
        .zip(b.chars())
        .fold(0, |d, (x, y)| if x == y { d } else { d + 1 });

    return Ok(distance);
}