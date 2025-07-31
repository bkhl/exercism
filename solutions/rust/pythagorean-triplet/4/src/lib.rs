pub fn find() -> Option<u32> {
    for a in 0..(1000u32/3){
        for b in 0..(1000/2) {
            let c = 1000 - (a + b);
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some(a * b * c)
            }
        }
    }
    None
}