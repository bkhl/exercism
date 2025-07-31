use std::fmt::Write;

pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb = String::new();
    if list.is_empty() {
        proverb
    } else {
        for pair in list.windows(2) {
            write!(proverb, "For want of a {} the {} was lost.\n", pair[0], pair[1]);
        }
        write!(proverb, "And all for the want of a {}.", list.first().unwrap());
        proverb
    }
}