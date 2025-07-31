pub fn reply(message: &str) -> &str {
    match message {
        m if m.is_silent() => "Fine. Be that way!",
        m if m.is_yell() => "Whoa, chill out!",
        m if m.is_question() => "Sure.",
        _ => "Whatever."
    }
}

trait Sentence {
    fn is_yell(&self) -> bool;
    fn is_question(&self) -> bool;
    fn is_silent(&self) -> bool;
}

impl Sentence for str {
    fn is_yell(&self) -> bool {
        self.chars().any(|c| c.is_alphabetic()) &&
            self.to_uppercase() == self
    }

    fn is_silent(&self) -> bool {
        self.trim().is_empty()
    }

    fn is_question(&self) -> bool {
        self.trim().ends_with("?")
    }
}