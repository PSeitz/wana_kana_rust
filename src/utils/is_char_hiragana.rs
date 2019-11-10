use constants::{HIRAGANA_END, HIRAGANA_START};
use crate::utils::is_char_in_range::*;
use crate::utils::is_char_long_dash::is_char_long_dash;

/// Tests a character. Returns true if the character is [Hiragana](https://en.wikipedia.org/wiki/Hiragana).
///
/// @param  {String} char character string to test
///

pub fn is_char_hiragana(char: char) -> bool {
    if is_char_long_dash(char) {
        return true;
    };
    return is_char_in_range(char, HIRAGANA_START, HIRAGANA_END);
}
