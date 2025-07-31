use std::collections::HashMap;

fn is_nucleotide(c: char) -> bool {
    match c {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}

pub fn nucleotide_counts(s: &str) -> Result<HashMap<char, usize>, &str> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    counts.insert('A', 0);
    counts.insert('C', 0);
    counts.insert('G', 0);
    counts.insert('T', 0);

    for c in s.chars() {
        if is_nucleotide(c) {
            *counts.get_mut(&c).unwrap() += 1;
        } else {
            return Err("Invalid DNA.");
        }
    }

    Ok(counts)
}

pub fn count(nucleotide: char, s: &str) -> Result<usize, &str> {
    let mut count = 0;

    if !is_nucleotide(nucleotide) {
        return Err("Invalid nucleotide.");
    }

    for c in s.chars() {
        if c == nucleotide {
            count += 1;
        } else if !is_nucleotide(c) {
            return Err("Invalid DNA.");
        }
    }

    Ok(count)
}