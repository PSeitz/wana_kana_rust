use crate::constants::{HIRAGANA_END, HIRAGANA_START};
use crate::utils::is_char_in_range::*;
use crate::utils::is_char_long_dash::is_char_long_dash;

/// Tests a character. Returns true if the character is [Hiragana](https://en.wikipedia.org/wiki/Hiragana).
pub fn is_char_hiragana(char: char) -> bool {
    if is_char_long_dash(char) {
        return true;
    };
    is_char_in_range(char, HIRAGANA_START, HIRAGANA_END)
}

#[test]
fn is_char_hiragana_test() {
    assert!(is_char_hiragana('な'));
    assert!(!is_char_hiragana('ナ'));
    assert!(!is_char_hiragana('n'));
    assert!(!is_char_hiragana('!'));
}
