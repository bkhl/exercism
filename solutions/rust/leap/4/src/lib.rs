pub fn is_leap_year(year: u64) -> bool {
    let mut result = year % 4 == 0;

    if result {
        result = year % 100 != 0
    }

    if ! result {
        result = year % 400 == 0
    }

    result
}
