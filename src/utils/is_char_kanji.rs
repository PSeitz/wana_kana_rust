use crate::utils::is_char_in_range::*;

static KANJI_START: u32 = 0x4E00;
static KANJI_END: u32 = 0x9FAF;

/// Tests a character. Returns true if the character is a CJK ideograph (kanji).
pub fn is_char_kanji(char: char) -> bool {
    is_char_in_range(char, KANJI_START, KANJI_END)
}

#[test]
fn is_char_kanji_test() {
    assert!(is_char_kanji('腹'));
    assert!(is_char_kanji('一')); // kanji for いち・1 - not a long hyphen
    assert!(!is_char_kanji('ー')); // long hyphen
    assert!(!is_char_kanji('は'));
    assert!(!is_char_kanji('ナ'));
    assert!(!is_char_kanji('n'));
    assert!(!is_char_kanji('!'));
}
