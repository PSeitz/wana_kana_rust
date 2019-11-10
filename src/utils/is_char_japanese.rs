use crate::constants::JAPANESE_RANGES;
use crate::utils::is_char_in_range::*;

/// Tests a character. Returns true if the character is [Katakana](https://en.wikipedia.org/wiki/Katakana).
///
/// @param  {String} char character string to test
///

pub fn is_char_japanese(char: char) -> bool {
    return JAPANESE_RANGES.iter().any(|el: &[u32; 2]| is_char_in_range(char, el[0], el[1]));
}
