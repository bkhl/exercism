use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T>(a: &[T], b: &[T]) -> bool
where
    T: PartialEq,
{
    for i in 0..(b.len() - a.len() + 1) {
        if a == &b[i..(i + a.len())] {
            return true;
        }
    }
    false
}

pub fn sublist<T>(a: &[T], b: &[T]) -> Comparison
where
    T: PartialEq,
{
    match a.len().cmp(&b.len()) {
        Ordering::Less => match is_sublist(a, b) {
            true => Comparison::Sublist,
            false => Comparison::Unequal,
        },
        Ordering::Greater => match is_sublist(b, a) {
            true => Comparison::Superlist,
            false => Comparison::Unequal,
        },
        Ordering::Equal => match a == b {
            true => Comparison::Equal,
            false => Comparison::Unequal,
        },
    }
}
