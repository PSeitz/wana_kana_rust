use crate::utils::is_char_english_punctuation::*;
use crate::utils::is_char_japanese_punctuation::*;

/// Tests a character. Returns true if the character is considered Japanese or English punctuation.

pub fn is_char_punctuation(char: char) -> bool {
    is_char_english_punctuation(char) || is_char_japanese_punctuation(char)
}
