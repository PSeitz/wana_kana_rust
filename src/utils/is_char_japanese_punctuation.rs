
use constants::JA_PUNCTUATION_RANGES;
use utils::is_char_inRange::*;

/**
 * Tests a character. Returns true if the character is considered English punctuation.
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn is_char_japanese_punctuation(char: char) -> bool {
  
  return JA_PUNCTUATION_RANGES.iter().any(([start, end]) => is_char_inRange(char, start, end));
}


