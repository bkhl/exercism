use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();
    for (score, letters) in input.iter() {
        for letter in letters {
            result.insert(letter.to_ascii_lowercase(), *score);
        }
    }
    result
}