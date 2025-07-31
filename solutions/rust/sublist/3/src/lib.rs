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
        Ordering::Less => if is_sublist(a, b) {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        },
        Ordering::Greater => if is_sublist(b, a) {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        },
        Ordering::Equal => if a == b {
            Comparison::Equal
        } else {
            Comparison::Unequal
        },
    }
}
