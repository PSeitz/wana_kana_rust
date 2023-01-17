/// Takes a character and a unicode range. Returns true if the char is in the range.
///
/// * `char` -  unicode character
///
/// * `start` - unicode start range
///
/// * `end` -   unicode end range

#[inline]
pub fn is_char_in_range(char: char, start: u32, end: u32) -> bool {
    start <= char as u32 && char as u32 <= end
}

#[test]
fn is_char_in_range_test() {
    use crate::constants::{HIRAGANA_END, HIRAGANA_START};
    assert_eq!(is_char_in_range('は', HIRAGANA_START, HIRAGANA_END), true);
    assert_eq!(is_char_in_range('d', HIRAGANA_START, HIRAGANA_END), false);
}
