use crate::constants::PROLONGED_SOUND_MARK;

/// Returns true if char is 'ー'
///
/// @param  {String} char to test
///

pub fn is_char_long_dash(char: char) -> bool {
    return char as u32 == PROLONGED_SOUND_MARK;
}
