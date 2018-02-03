
import { JA_PUNCTUATION_RANGES } from '../constants';
use utils::is_char_inRange::*;

/**
 * Tests a character. Returns true if the character is considered English punctuation.
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn is_char_japanese_punctuation(char: char) -> bool {
  
  return JA_PUNCTUATION_RANGES.some(([start, end]) => is_char_inRange(char, start, end));
}


