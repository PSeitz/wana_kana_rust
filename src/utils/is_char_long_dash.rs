use crate::constants::PROLONGED_SOUND_MARK;

/// Returns true if char is 'ー'
pub fn is_char_long_dash(char: char) -> bool {
    char as u32 == PROLONGED_SOUND_MARK
}

#[test]
fn is_char_long_dash_test() {
    assert_eq!(is_char_long_dash('ー'), true);
    assert_eq!(is_char_long_dash('-'), false);
    assert_eq!(is_char_long_dash('f'), false);
    assert_eq!(is_char_long_dash('ふ'), false);
}
