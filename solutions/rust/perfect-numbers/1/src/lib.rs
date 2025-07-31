#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Result<Classification, &'static str> {
    if num == 0 {
        return Err("Number must be positive");
    }

    let sum: u64 = (1..(num / 2 + 1)).filter(|d| num % d == 0).sum();

    if sum < num {
        Ok(Classification::Deficient)
    } else if sum > num {
        Ok(Classification::Abundant)
    } else {
        Ok(Classification::Perfect)
    }
}