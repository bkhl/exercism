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
    a.is_empty() || b.windows(a.len()).any(|w| w == a)
}

pub fn sublist<T>(a: &[T], b: &[T]) -> Comparison
where
    T: PartialEq,
{
    use Comparison::*;

    match (is_sublist(a, b), is_sublist(b, a)) {
        (true, true) => Equal,
        (true, false) => Sublist,
        (false, true) => Superlist,
        (false, false) => Unequal,
    }
}
