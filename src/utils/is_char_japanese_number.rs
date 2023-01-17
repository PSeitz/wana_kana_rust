use crate::constants::ZENKAKU_NUMBERS;
use crate::utils::is_char_in_range::*;

/// Tests a character. Returns true if the character is considered a Zenkaku number(０-９)

pub fn is_char_japanese_number(char: char) -> bool {
    is_char_in_range(char, ZENKAKU_NUMBERS[0], ZENKAKU_NUMBERS[1])
}
