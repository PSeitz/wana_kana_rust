use crate::utils::is_char_hiragana::*;
use crate::utils::is_char_katakana::*;

/// Tests a character. Returns true if the character is [Hiragana](https://en.wikipedia.org/wiki/Hiragana) or [Katakana](https://en.wikipedia.org/wiki/Katakana).

pub fn is_char_kana(char: char) -> bool {
    is_char_hiragana(char) || is_char_katakana(char)
}

#[test]
fn is_char_kana_test() {
    assert_eq!(is_char_kana('は'), true);
    assert_eq!(is_char_kana('ナ'), true);
    assert_eq!(is_char_kana('n'), false);
    assert_eq!(is_char_kana('!'), false);
    assert_eq!(is_char_kana('-'), false);
    assert_eq!(is_char_kana('ー'), true);
}
