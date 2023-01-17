use crate::constants::JA_PUNCTUATION_RANGES;
use crate::utils::is_char_in_range::*;

/// Tests a character. Returns true if the character is considered japanese punctuation.

pub fn is_char_japanese_punctuation(char: char) -> bool {
    JA_PUNCTUATION_RANGES
        .iter()
        .any(|el: &[u32; 2]| is_char_in_range(char, el[0], el[1]))
}
