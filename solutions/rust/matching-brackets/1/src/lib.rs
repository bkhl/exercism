static LEFT_BRACKETS: &'static [char] = &['(', '[', '{'];
static RIGHT_BRACKETS: &'static [char] = &[')', ']', '}'];

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
            if let Some(i) = LEFT_BRACKETS.iter().position(|b| *b == c) {
                stack.push(i);
            } else if let Some(i) = RIGHT_BRACKETS.iter().position(|b| *b == c) {
                if stack.pop() != Some(i) {
                    return Unbalanced;
                }
            }
        }

        if stack.is_empty() {
            Balanced
        } else {
            Unbalanced
        }
    }
}
