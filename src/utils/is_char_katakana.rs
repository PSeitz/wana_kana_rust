use crate::constants::{KATAKANA_END, KATAKANA_START};
use crate::utils::is_char_in_range::*;

/// Tests a character. Returns true if the character is [Katakana](https://en.wikipedia.org/wiki/Katakana).

pub fn is_char_katakana(char: char) -> bool {
    is_char_in_range(char, KATAKANA_START, KATAKANA_END)
}

#[test]
fn is_char_katakana_test() {
    assert!(is_char_katakana('ナ'));
    assert!(!is_char_katakana('は'));
    assert!(!is_char_katakana('n'));
    assert!(!is_char_katakana('!'));
}
