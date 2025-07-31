pub fn reply(message: &str) -> &str {
    if message.is_yell() {
        "Whoa, chill out!"
    } else if message.is_question() {
        "Sure."
    } else if message.is_silent() {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}

trait Sentence {
    fn is_yell(&self) -> bool;
    fn is_question(&self) -> bool;
    fn is_silent(&self) -> bool;
}

impl Sentence for str {
    fn is_yell(&self) -> bool {
        let mut has_letters = false;
        let mut all_uppercase = true;

        for c in self.chars() {
            if c.is_alphabetic() {
                has_letters = true;
                if c.is_lowercase() {
                    all_uppercase  = false
                }
            }
        }

        if has_letters && all_uppercase  {
            true
        } else {
            false
        }
    }

    fn is_silent(&self) -> bool {
        for c in self.chars() {
            if ! c.is_whitespace() {
                return false
            }
        }
        true
    }

    fn is_question(&self) -> bool {
        for c in self.chars().rev() {
            if c == '?' {
                return true
            } else if c.is_whitespace() {
                continue
            } else {
                break
            }
        }
        false
    }
}