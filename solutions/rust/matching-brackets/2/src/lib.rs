#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;

static BRACKETS: phf::Map<char, char> = phf_map! {
    '(' => ')',
    '[' => ']',
    '{' => '}',
};

#[derive(PartialEq)]
pub enum Brackets {
    Balanced,
    Unbalanced,
}

impl Brackets {
    pub fn are_balanced(&self) -> bool {
        *self == Brackets::Balanced
    }
}

impl From<&'static str> for Brackets {
    fn from(input: &str) -> Self {
        use Brackets::*;

        let mut stack = vec![];

        for c in input.chars() {
            if Some(&c) == stack.last() {
                stack.pop();
            } else if let Some(left_bracket) = BRACKETS.get(&c) {
                stack.push(*left_bracket)
            } else if BRACKETS.values().any(|x| x == &c) {
                return Unbalanced;
            }
        }

        if stack.is_empty() {
            Balanced
        } else {
            Unbalanced
        }
    }
}
