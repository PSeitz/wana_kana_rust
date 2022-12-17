use crate::constants::KANA_SLASH_DOT;

/// Tests if char is '・'
#[inline]
pub fn is_char_slash_dot(char: char) -> bool {
    char as u32 == KANA_SLASH_DOT
}

#[test]
fn is_char_slash_dot_test() {
    assert_eq!(is_char_slash_dot('・'), true);
    assert_eq!(is_char_slash_dot('/'), false);
}
