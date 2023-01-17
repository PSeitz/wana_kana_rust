use crate::constants::{KATAKANA_END, KATAKANA_START};
use crate::utils::is_char_in_range::*;

/// Tests a character. Returns true if the character is [Katakana](https://en.wikipedia.org/wiki/Katakana).

pub fn is_char_katakana(char: char) -> bool {
    is_char_in_range(char, KATAKANA_START, KATAKANA_END)
}

#[test]
fn is_char_katakana_test() {
    assert_eq!(is_char_katakana('ナ'), true);
    assert_eq!(is_char_katakana('は'), false);
    assert_eq!(is_char_katakana('n'), false);
    assert_eq!(is_char_katakana('!'), false);
}
