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
    use Comparison::*;

    match a.len().cmp(&b.len()) {
        Ordering::Less => if is_sublist(a, b) {
            Sublist
        } else {
            Unequal
        },
        Ordering::Greater => if is_sublist(b, a) {
            Superlist
        } else {
            Unequal
        },
        Ordering::Equal => if a == b {
            Equal
        } else {
            Unequal
        },
    }
}
