pub fn map_closure<F, T>(input: Vec<T>, f: F) -> Vec<T>
where
    F: FnMut(T) -> T,
{
    input.into_iter().map(f).collect()
}

pub fn map_function<F, T>(input: Vec<T>, f: F) -> Vec<T>
where
    F: FnMut(T) -> T,
{
    map_closure(input, f)
}