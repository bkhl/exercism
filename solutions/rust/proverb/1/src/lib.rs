use std::fmt::Write;

pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb = String::new();
    if list.is_empty() {
        proverb
    } else {
        for (wanted, lost) in pairs(list.clone()) {
            write!(proverb, "For want of a {} the {} was lost.\n", wanted, lost);
        }
        write!(proverb, "And all for the want of a {}.", list.first().unwrap());
        proverb
    }
}

pub fn pairs(list : Vec<&str>) -> Vec<(&str, &str)> {
    let mut result: Vec<(&str, &str)> = Vec::new();
    for i in 0..(list.len() - 1) {
        result.push((list[i], list[i + 1]))
    }
    result
}