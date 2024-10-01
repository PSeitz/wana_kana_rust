use crate::constants::PROLONGED_SOUND_MARK;

/// Returns true if char is 'ー'
pub fn is_char_long_dash(char: char) -> bool {
    char as u32 == PROLONGED_SOUND_MARK
}

#[test]
fn is_char_long_dash_test() {
    assert!(is_char_long_dash('ー'));
    assert!(!is_char_long_dash('-'));
    assert!(!is_char_long_dash('f'));
    assert!(!is_char_long_dash('ふ'));
}
