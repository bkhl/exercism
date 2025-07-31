pub fn find() -> Option<u32> {
    for a in 0..1000 {
        for b in 0..a {
            let c = 1000 - (a + b);
            if (a as u32).pow(2) + (b as u32).pow(2) == (c as u32).pow(2) {
                return Some(a * b * c)
            }
        }
    }
    None
}