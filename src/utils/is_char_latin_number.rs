use crate::constants::LATIN_NUMBERS;
use crate::utils::is_char_in_range::*;

/// Tests a character. Returns true if the character is considered a latin number. 0-9

pub fn is_char_latin_number(char: char) -> bool {
    is_char_in_range(char, LATIN_NUMBERS[0], LATIN_NUMBERS[1])
}
