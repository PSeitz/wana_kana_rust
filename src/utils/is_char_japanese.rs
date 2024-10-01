//! Tests a character. Returns true if the character is in any unicode range used by japanese.
use crate::constants::JAPANESE_RANGES;
use crate::utils::is_char_in_range::*;

/// Tests a character. Returns true if the character is in any unicode range used by japanese.
pub fn is_char_japanese(char: char) -> bool {
    JAPANESE_RANGES
        .iter()
        .any(|el: &[u32; 2]| is_char_in_range(char, el[0], el[1]))
}

#[test]
fn is_char_japanese_test() {
    assert!(is_char_japanese('１'));
    assert!(is_char_japanese('ナ'));
    assert!(is_char_japanese('は'));
    assert!(is_char_japanese('缶'));
    assert!(is_char_japanese('〜'));
    assert!(is_char_japanese('ｎ'));
    assert!(is_char_japanese('Ｋ'));
    assert!(!is_char_japanese('1'));
    assert!(!is_char_japanese('n'));
    assert!(!is_char_japanese('K'));
    assert!(!is_char_japanese('!'));
}
