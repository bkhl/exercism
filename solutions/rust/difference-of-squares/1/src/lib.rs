pub fn square_of_sum(n: usize) -> usize {
    sum(n).pow(2)
}

fn sum(n: usize) -> usize {
    match n {
        0 => 0,
        _ => n + sum(n - 1)
    }
}

pub fn sum_of_squares(n: usize) -> usize {
    match n {
        0 => 0,
        _ => n.pow(2) + sum_of_squares(n - 1)
    }
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}