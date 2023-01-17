/// Tests a character. Returns true if the character is considered English punctuation.
///
/// * `char` - character to test
use crate::constants::EN_PUNCTUATION_RANGES;
use crate::utils::is_char_in_range::*;

pub fn is_char_english_punctuation(char: char) -> bool {
    EN_PUNCTUATION_RANGES
        .iter()
        .any(|el: &[u32; 2]| is_char_in_range(char, el[0], el[1]))
}
