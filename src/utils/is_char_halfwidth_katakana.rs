use crate::constants::{HALFWIDTH_KATAKANA_END, HALFWIDTH_KATAKANA_START};
use crate::utils::is_char_in_range::*;

/// Tests a character. Returns true if the character is [Half-width Katakana](https://en.wikipedia.org/wiki/Half-width_kana).

pub fn is_char_halfwidth_katakana(char: char) -> bool {
    is_char_in_range(char, HALFWIDTH_KATAKANA_START, HALFWIDTH_KATAKANA_END)
}

#[test]
fn is_char_halfwidth_katakana_test() {
    assert!(is_char_halfwidth_katakana('｡'));
    assert!(is_char_halfwidth_katakana('ｦ'));
    assert!(is_char_halfwidth_katakana('ﾟ'));
    assert!(is_char_halfwidth_katakana('｢'));
    assert!(is_char_halfwidth_katakana('｣'));

    assert!(!is_char_halfwidth_katakana('ナ'));
    assert!(!is_char_halfwidth_katakana('は'));
    assert!(!is_char_halfwidth_katakana('n'));
    assert!(!is_char_halfwidth_katakana('!'));
}
