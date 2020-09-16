use crate::utils::is_char_in_range::*;

static KANJI_START: u32 = 0x4E00;
static KANJI_END: u32 = 0x9FAF;

/// Tests a character. Returns true if the character is a CJK ideograph (kanji).
pub fn is_char_kanji(char: char) -> bool {
    is_char_in_range(char, KANJI_START, KANJI_END)
}

#[test]
fn is_char_kanji_test() {
    assert_eq!(is_char_kanji('腹'), true);
    assert_eq!(is_char_kanji('一'), true); // kanji for いち・1 - not a long hyphen
    assert_eq!(is_char_kanji('ー'), false); // long hyphen
    assert_eq!(is_char_kanji('は'), false);
    assert_eq!(is_char_kanji('ナ'), false);
    assert_eq!(is_char_kanji('n'), false);
    assert_eq!(is_char_kanji('!'), false);
}
