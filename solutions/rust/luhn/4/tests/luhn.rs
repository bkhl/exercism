extern crate luhn;

use luhn::*;

#[test]
fn single_digit_string_is_invalid() {
    assert!(!"1".valid());
}

#[test]
fn single_zero_string_is_invalid() {
    assert!(!"0".valid());
}

#[test]
fn simple_valid_sin() {
    assert!(" 5 9 ".valid());
}

#[test]
fn valid_canadian_sin_is_valid() {
    assert!("046 454 286".valid());
}

#[test]
fn invalid_canadian_sin_is_invalid() {
    assert!(!"046 454 287".valid());
}

#[test]
fn invalid_credit_card_is_invalid() {
    assert!(!"8273 1232 7352 0569".valid());
}

#[test]
fn strings_that_contain_non_digits_are_invalid() {
    assert!(!"046a 454 286".valid());
}

#[test]
fn punctuation_is_invalid() {
    assert!(!"055-444-285".valid());
}

#[test]
fn symbols_are_invalid() {
    assert!(!"055£ 444$ 285".valid());
}

#[test]
fn single_digit_with_space_is_invalid() {
    assert!(!" 0".valid());
}

#[test]
fn lots_of_zeros_are_valid() {
    assert!(" 00000".valid());
}

#[test]
fn another_valid_sin() {
    assert!("055 444 285".valid());
}

#[test]
fn nine_doubled_is_nine() {
    assert!("091".valid());
}

#[test]
fn u64_valid() {
    assert!(055444285_u64.valid());
}