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
    assert_eq!(is_char_japanese('１'), true);
    assert_eq!(is_char_japanese('ナ'), true);
    assert_eq!(is_char_japanese('は'), true);
    assert_eq!(is_char_japanese('缶'), true);
    assert_eq!(is_char_japanese('〜'), true);
    assert_eq!(is_char_japanese('ｎ'), true);
    assert_eq!(is_char_japanese('Ｋ'), true);
    assert_eq!(is_char_japanese('1'), false);
    assert_eq!(is_char_japanese('n'), false);
    assert_eq!(is_char_japanese('K'), false);
    assert_eq!(is_char_japanese('!'), false);
}
