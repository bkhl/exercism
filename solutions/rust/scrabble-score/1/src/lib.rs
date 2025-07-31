macro_rules! letter_match {
    ($c:ident, $l:ident) => (
        $c == stringify!($l)
    );
    ($c:ident, $l:ident, $($m:ident), +) => (
        letter_match!($c, $($m), +) || $c == stringify!($l)
    );
}

pub fn score(word: &str) -> u32 {
    let mut score = 0;

    for letter in word.chars().map(|c| c.to_uppercase().to_string()) {
        if letter_match!(letter, A, E, I, O, U, L, N, R, S, T) {
            score += 1;
        } else if letter_match!(letter, D, G) {
            score += 2;
        } else if letter_match!(letter, B, C, M, P) {
            score += 3;
        } else if letter_match!(letter, F, H, V, W, Y) {
            score += 4;
        } else if letter_match!(letter, K) {
            score += 5;
        } else if letter_match!(letter, J, X) {
            score += 8
        } else if letter_match!(letter, Q, Z) {
            score += 10
        }
    }

    score
}