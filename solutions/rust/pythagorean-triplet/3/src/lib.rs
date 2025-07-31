pub fn find() -> Option<u32> {
    for a in 0..1000_u32 {
        for b in 0..a {
            let c = 1000 - (a + b);
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some(a * b * c)
            }
        }
    }
    None
}