
import { EN_PUNCTUATION_RANGES } from '../constants';
use utils::isCharInRange::*;

/**
 * Tests a character. Returns true if the character is considered English punctuation.
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn isCharEnglishPunctuation(char: char) -> bool {
  
  return EN_PUNCTUATION_RANGES.some(([start, end]) => isCharInRange(char, start, end));
}

export default isCharEnglishPunctuation;
