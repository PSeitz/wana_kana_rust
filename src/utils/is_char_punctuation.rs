use utils::is_char_english_punctuation::*;
use utils::is_char_japanese_punctuation::*;


///Tests a character. Returns true if the character is considered Japanese or English punctuation.
///
///@param  {String} char character string to test
///

pub fn is_char_punctuation(char: char) -> bool {
    return is_char_english_punctuation(char) || is_char_japanese_punctuation(char);
}
