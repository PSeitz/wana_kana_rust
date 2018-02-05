use constants::JA_PUNCTUATION_RANGES;
use utils::is_char_in_range::*;


///Tests a character. Returns true if the character is considered English punctuation.
///
///@param  {String} char character string to test
///

pub fn is_char_japanese_punctuation(char: char) -> bool {
    return JA_PUNCTUATION_RANGES
        .iter()
        .any(|el: &[u32; 2]| is_char_in_range(char, el[0], el[1]));
}
