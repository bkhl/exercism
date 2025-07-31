pub fn find() -> Option<u32> {
    for a in 0_u32..1000 {
        for b in 0_u32..a {
            let c = 1000_u32 - (a + b);
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some(a * b * c)
            }
        }
    }
    None
}