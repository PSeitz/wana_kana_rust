use constants::KANA_SLASH_DOT;

/// Tests if char is '・'
///
/// @param  {String} char
///

pub fn is_char_slash_dot(char: char) -> bool {
    return char as u32 == KANA_SLASH_DOT;
}
